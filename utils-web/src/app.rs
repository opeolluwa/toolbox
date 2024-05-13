use crate::pages::authorize::{Login, MultiFactorAuth, ResetAccount, SecurityQuestion};
use crate::pages::dashboard::Dashboard;
use crate::pages::home::Home;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {

        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <Home/> }/>
                <Route path="/auth/login" view=  move || view! { <Login/> }/>
                <Route path="/auth/sescurity-question" view=  move || view! { <SecurityQuestion/> }/>
                <Route path="/auth/reset" view=  move || view! { <ResetAccount/> }/>
                <Route path="/auth/mfa" view=  move || view! { <MultiFactorAuth/> }/>
                <Route path="/dashboard" view=  move || view! { <Dashboard/> }/>

            </Routes>
        </Router>
    }
}
