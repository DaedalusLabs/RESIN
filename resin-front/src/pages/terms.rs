use yew::prelude::*;


#[function_component(TermsAndConditions)]
pub fn terms_and_conditions() -> Html {
    html! {
        <div class="bg-white p-4 max-h-96 overflow-y-auto">
            <h1>{"Terms and Conditions for Daedalus Labs Ltd."}</h1>
            <div>
                <h2>{"Introduction"}</h2>
                <p>{"Welcome to Daedalus Labs Ltd. These Terms and Conditions govern your use of our services, products, and website. By accessing or using our services, you agree to comply with and be bound by these Terms. If you do not agree with any part of these Terms, you must not use our services."}</p>
            </div>
            <div>
                <h2>{"Definitions"}</h2>
                <p>{"Company refers to Daedalus Labs Ltd."}</p>
                <p>{"Services refers to all products, services, and content provided by the Company."}</p>
                <p>{"User refers to any individual or entity that accesses or uses the Services."}</p>
            </div>
            <div>
                <h2>{"Acceptance of Terms"}</h2>
                <p>{"By using our Services, you confirm that you are at least 18 years old or have the consent of a parent or guardian. If you are using the Services on behalf of an organization, you represent that you have the authority to bind that organization to these Terms."}</p>
            </div>
            <div>
                <h2>{"Modifications to Terms"}</h2>
                <p>{"Daedalus Labs Ltd. reserves the right to modify these Terms at any time. Changes will be effective immediately upon posting on our website. Your continued use of the Services after any changes constitutes your acceptance of the new Terms."}</p>
            </div>
            <div>
                <h2>{"User Responsibilities"}</h2>
                <p>{"Users agree to:"}</p>
                <ul>
                    <li>{"Provide accurate and complete information when creating an account or using our Services."}</li>
                    <li>{"Use the Services only for lawful purposes and in accordance with applicable laws."}</li>
                    <li>{"Not engage in any conduct that restricts or inhibits anyone's use or enjoyment of the Services."}</li>
                </ul>
            </div>
            <div>
                <h2>{"Intellectual Property"}</h2>
                <p>{"All content, trademarks, and other intellectual property associated with Daedalus Labs Ltd. are the property of the Company or its licensors. Users are granted a limited, non-exclusive, non-transferable license to use the Services for personal, non-commercial purposes."}</p>
            </div>
            <div>
                <h2>{"Limitation of Liability"}</h2>
                <p>{"To the fullest extent permitted by law, Daedalus Labs Ltd. shall not be liable for any indirect, incidental, special, consequential, or punitive damages arising from your use of the Services. Our total liability to you for any claims arising out of these Terms or your use of the Services shall not exceed the amount paid by you, if any, for the Services."}</p>
            </div>
            <div>
                <h2>{"Indemnification"}</h2>
                <p>{"You agree to indemnify, defend, and hold harmless Daedalus Labs Ltd., its affiliates, and their respective officers, directors, employees, and agents from any claims, losses, liabilities, damages, costs, or expenses (including reasonable attorneys' fees) arising out of your use of the Services or violation of these Terms."}</p>
            </div>
            <div>
                <h2>{"Governing Law"}</h2>
                <p>{"These Terms shall be governed by and construed in accordance with the laws of [Your Jurisdiction]. Any disputes arising from these Terms shall be resolved in the courts of [Your Jurisdiction]."}</p>
            </div>
            <div>
                <h2>{"Contact Information"}</h2>
                <p>{"For any questions or concerns regarding these Terms, please contact us at:"}</p>
                <p>{"Daedalus Labs Ltd."}</p>
                <p>{"[Your Address]"}</p>
                <p>{"[Your Email]"}</p>
                <p>{"[Your Phone Number]"}</p>
            </div>
            <div>
                <h2>{"Entire Agreement"}</h2>
                <p>{"These Terms constitute the entire agreement between you and Daedalus Labs Ltd. regarding your use of the Services and supersede any prior agreements or understandings. No waiver of any provision of these Terms shall be deemed a further or continuing waiver of such provision or any other provision."}</p>
            </div>
        </div>
    }
}


#[function_component(FrequentlyAskedQuestions)]
pub fn frequently_asked_questions() -> Html {
    html! {
        <div>
            <h1>{"Frequently Asked Questions about RESIN"}</h1>
            <QuestionDrawer 
                title={"Why choose Resin's over traditional real estate investment trusts?"} 
                content={"Resin's REIT democratizes real estate investment, offering lower entry barriers and minimizing extra costs. Unlike traditional REITs, which often cater to the affluent with high minimum investments and are less liquid, Resin ensures accessibility and liquidity, allowing more people to invest and manage their investments without the heavy fees and restrictions of conventional methods."}
                    />
            <QuestionDrawer 
                title={"What makes Resin's REIT an attractive option for property investment and ownership?"} 
                content={"PLACEHOLDER"}
                    />
            <QuestionDrawer 
                title={"How does Resin's REIT accommodate the needs of digital nomads and Bitcoin investors?"} 
                content={"LOREWM IPSUM"}
                    />
            <QuestionDrawer 
                title={"How does Resin use Bitcoin to facilitate real estate investments?"} 
                content={"LOREWM IPSUM"}
                    />
        </div>
    }
}

#[derive(Clone, PartialEq, Properties, Eq)]
pub struct QuestionDrawerProps {
    pub title: String,
    pub content: String,
}

#[function_component(QuestionDrawer)]
pub fn question_drawer(props: &QuestionDrawerProps) -> Html {
    let QuestionDrawerProps { title, content } = props;
    let is_open = use_state(|| false);
    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };
    let drawer_class = if *is_open { 
        "block transition ease-in duration-100 transform  max-h-96 overflow-y-auto"
    } else {
        "block transition ease-out duration-100 transform max-h-0 overflow-y-hidden"
    };
    html! {
       <div class="bg-white p-4 max-h-96 overflow-y-auto">
           <div class="flex flex-row justify-between">
               <h2>{&title}</h2>
               <button {onclick}>
                   <lucide_yew::ChevronDown class="w-6 h-6 text-gray-500" />
               </button>
            </div>
           <div class={drawer_class}>
            <p>{&content}</p>
           </div>
       </div>
    }
}
