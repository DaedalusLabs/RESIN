use yew::prelude::*;
#[function_component(SettingsPage)]
pub fn settings_page() -> Html {
    html! {
        <div class="max-w-2xl space-y-8">
            <div class="space-y-6 divide-y divide-gray-200">
                <CurrencyForm />
                <LanguageForm />
                <PushNotificationToggle />
            </div>
        </div>
    }
}

#[function_component(CurrencyForm)]
pub fn currency_form() -> Html {
    html! {
        <div class="pt-6 first:pt-0">
            <h4 class="text-sm font-semibold my-2 text-[#444446]">
                {"Currency"}
            </h4>
            <form class="space-y-4">
                <div class="flex flex-col space-y-2">
                    <label for="currency" class="text-sm font-medium text-gray-700">{"Select your preferred currency"}</label>
                    <select id="currency" name="currency" 
                        class="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-bitcoin-orange focus:border-bitcoin-orange">
                        <option value="btc">{"BTC"}</option>
                        <option value="eur">{"EUR"}</option>
                        <option value="SRD">{"SRD"}</option>
                        <option value="usd">{"USD"}</option>
                    </select>
                </div>
            </form>
        </div>
    }
}

#[function_component(LanguageForm)]
pub fn language_form() -> Html {
    html! {
        <div class="pt-6">
            <h4 class="text-sm font-semibold my-2 text-[#444446]">
                {"Language"}
            </h4>
            <form class="space-y-4">
                <div class="flex flex-col space-y-2">
                    <label for="language" class="text-sm font-medium text-gray-700">{"Select your preferred language"}</label>
                    <select id="language" name="language" 
                        class="w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-bitcoin-orange focus:border-bitcoin-orange">
                        <option value="en">{"English"}</option>
                        <option value="es">{"Spanish"}</option>
                        <option value="fr">{"Dutch"}</option>
                    </select>
                </div>
            </form>
        </div>
    }
}

#[function_component(PushNotificationToggle)]
pub fn push_notification_toggle() -> Html {
    html! {
        <div class="pt-6">
            <h4 class="text-sm font-semibold my-2 text-[#444446]">
                {"Push Notifications"}
            </h4>
            <form class="space-y-4">
                <div class="flex items-center">
                    <input type="checkbox" id="push-notifications" name="push-notifications" 
                           class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded" />
                    <label for="push-notifications" class="ml-3 text-sm font-medium text-gray-700">
                        {"Enable push notifications"}
                    </label>
                </div>
            </form>
        </div>
    }
}
