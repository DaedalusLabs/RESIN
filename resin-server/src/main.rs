static RELAYS: [&str; 2] = ["wss://relay.illuminodes.com", "wss://relay.illuminodes.com"];

static NOSTR_KEYS: std::sync::LazyLock<nostro2_signer::keypair::NostrKeypair> =
    std::sync::LazyLock::new(|| {
        let keypair = nostro2_signer::keypair::NostrKeypair::try_from(
            std::env::var("RESIN_KEY")
                .expect("RESIN_KEY has not been set")
                .as_str(),
        )
        .expect("Invalid keypair");
        keypair
    });

static APP_ID: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| std::env::var("UT_APP_ID").expect("UT_APP_ID has not been set"));

static API_KEY: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| std::env::var("UT_API_KEY").expect("UT_API_KEY has not been set"));

static REQWEST_CLIENT: std::sync::LazyLock<reqwest::Client> = std::sync::LazyLock::new(|| {
    let mut header_map = reqwest::header::HeaderMap::new();
    header_map.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    let api_key = std::env::var("UT_API_KEY").expect("UT_API_KEY has not been set");
    header_map.insert(
        "x-uploadthing-api-key",
        reqwest::header::HeaderValue::from_str(&api_key).expect("API_KEY has not been set"),
    );
    reqwest::Client::builder()
        .default_headers(header_map)
        .build()
        .expect("Failed to build reqwest client")
});

#[derive(Clone)]
pub struct UtSigner {
    client: &'static reqwest::Client,
    keys: &'static nostro2_signer::keypair::NostrKeypair,
    app_id: &'static str,
    api_key: &'static str,
}
impl UtSigner {
    /// Register a url for a file upload 
    ///
    /// # Errors 
    ///
    /// Returns an error if the request cannot be sent. 
    pub async fn register_url(
        &self,
        form: upload_things::UtRecord,
    ) -> Result<reqwest::Response, &'static str> {
        let region: &str = upload_things::UploadRegion::UsWestSeattle.alias();
        let url = format!("https://{region}.ingest.uploadthing.com/route-metadata");
        let post_request = self.client.post(&url).body(form.to_string());
        post_request.send().await.map_err(|e| {
            eprintln!("Failed to send request: {e:?}");
            "Failed to send request"
        })
    }
    /// Sign a url for a file upload 
    ///
    /// # Errors
    ///
    /// Returns an error if the file key cannot be generated, the presigned url cannot be
    /// generated, or the presigned url cannot be signed
    pub async fn sign_url(
        &self,
        payload: upload_things::UtRequest,
        pubkey: String,
    ) -> Result<nostro2::note::NostrNote, &'static str> {
        let mut unsigned = upload_things::UtPreSignedUrl::default();
        unsigned
            .generate_file_key(self.app_id.to_string())
            .map_err(|e| {
                eprintln!("Failed to generate file key: {e:?}");
                "Failed to generate file key"
            })?;
        unsigned
            .presigned_url(payload, self.api_key.to_string(), self.app_id.to_string())
            .map_err(|e| {
                eprintln!("Failed to generate presigned url: {e:?}");
                "Failed to generate presigned url"
            })?;
        let ut_record = upload_things::UtRecord {
            file_keys: vec![unsigned.file_key.clone()],
            ..Default::default()
        };
        self.register_url(ut_record).await.map_err(|e| {
            eprintln!("Failed to register url: {e:?}");
            "Failed to register url"
        })?;

        let mut signed = nostro2::note::NostrNote {
            kind: 20421,
            content: serde_json::to_string(&unsigned).map_err(|e| {
                eprintln!("Failed to serialize presigned url: {e:?}");
                "Failed to serialize presigned url"
            })?,
            pubkey: self.keys.public_key(),
            ..Default::default()
        };
        self.keys
            .sign_nip_44_encrypted(&mut signed, pubkey)
            .map_err(|e| {
                eprintln!("Failed to sign presigned url: {e:?}");
                "Failed to sign presigned url"
            })?;
        Ok(signed)
    }
}
impl Default for UtSigner {
    fn default() -> Self {
        Self {
            client: &REQWEST_CLIENT,
            keys: &NOSTR_KEYS,
            app_id: &APP_ID,
            api_key: &API_KEY,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nostr_bot = nostro2_relay::pool::NostrPool::new(&RELAYS).await;
    let uploader = UtSigner::default();
    let mut filter = nostro2::subscriptions::NostrSubscription {
        kinds: vec![21059, 20420].into(),
        ..Default::default()
    };
    filter.add_tag("#p", &NOSTR_KEYS.public_key());
    println!("Subscribing to relay {}", uploader.keys.public_key());

    nostr_bot.send(&filter).await?;
    println!("Subscribed to relay");

    while let Some(msg) = nostr_bot.recv().await {
        let nostro2::relay_events::NostrRelayEvent::NewNote(_, _id, encrypted_giftwrap) = msg else {
            continue;
        };
        let Ok(Ok(nostro2::note::NostrNote {
            content, pubkey, ..
        })) = uploader
            .keys
            .decrypt_nip_44_content(&encrypted_giftwrap)
            .map(|x| x.parse())
        else {
            eprintln!("Failed to decrypt message");
            continue;
        };
        if encrypted_giftwrap.kind == 20420 {
            println!("Received upload req: {}", &pubkey);
            let Ok(content): Result<upload_things::UtRequest, _> = content.try_into() else {
                eprintln!("Failed to deserialize message");
                continue;
            };

            let Ok(presigned_url_note) = uploader.sign_url(content, pubkey).await else {
                eprintln!("Failed to sign url");
                continue;
            };
            nostr_bot.send(&presigned_url_note).await?;
            continue;
        }
        if encrypted_giftwrap.kind == 21059 {
            println!("Received dm resp: {}", &pubkey);
            let id_pubkey = NOSTR_KEYS.public_key();
            let mut unsigned_kind_14 = nostro2::note::NostrNote {
                kind: 14,
                content: r"Thank you for contacting WIOSO! 
                We are not yet live, but we appreciate you reaching out if interested."
                    .to_string(),
                pubkey: id_pubkey.clone(),
                ..Default::default()
            };
            unsigned_kind_14.tags.add_pubkey_tag(&pubkey);
            unsigned_kind_14
                .tags
                .add_custom_tag(nostro2::tags::NostrTag::Custom("subject"), "none");
            let mut sealed_note = nostro2::note::NostrNote {
                kind: 13,
                content: unsigned_kind_14.to_string(),
                pubkey: id_pubkey.clone(),
                ..Default::default()
            };
            NOSTR_KEYS
                .sign_nip_44_encrypted(&mut sealed_note, pubkey.clone())
                .expect("Failed to sign note");
            let mut giftwrap = nostro2::note::NostrNote {
                kind: 21059,
                content: sealed_note.to_string(),
                pubkey: id_pubkey.clone(),
                ..Default::default()
            };
            NOSTR_KEYS
                .sign_nip_44_encrypted(&mut giftwrap, pubkey)
                .expect("couldnt sign giftwrap");
            nostr_bot
                .send(&giftwrap)
                .await
                .expect("couldnt send giftwrap");
        }
    }
    eprintln!("Relay closed");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn ping() {
        let mut new_note = nostro2::note::NostrNote {
            kind: 20421,
            content: "ping".to_string(),
            pubkey: NOSTR_KEYS.public_key().to_string(),
            ..Default::default()
        };
        let nostr_bot = nostro2_relay::pool::NostrPool::new(&RELAYS).await;
        NOSTR_KEYS
            .sign_nip_44_encrypted(
                &mut new_note,
                "f117246dd238a6023896d0f8047fa32d8f14e471914514a21739689ff7c39b76".to_string(),
            )
            .unwrap();
        nostr_bot.send(&new_note).await.unwrap();

        while let Some(msg) = nostr_bot.recv().await {
            let nostro2::relay_events::NostrRelayEvent::SentOk(_, _, did_send, _) = msg else {
                continue;
            };
            println!("Received message: {:?}", msg);
            println!("Send message {did_send}");
            break;
        }
    }
}
