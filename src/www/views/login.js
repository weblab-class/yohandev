import { POST } from "../utils";

const CLIENT_ID = "825478233856-jub75iqs08474an082n9hptsj94tses3.apps.googleusercontent.com";

export function LoginButton() {
    // Register callbacks:
    window.onLogin = ({ credential }) => {
        POST("/api/login", { credential }).then((res) => {
            console.log(res);
        });
    };
    // Generated from:
    // https://developers.google.com/identity/gsi/web/tools/configurator
    return (
        <>
            <div id="g_id_onload"
                data-client_id={CLIENT_ID}
                data-context="signup"
                data-ux_mode="popup"
                data-callback="onLogin"
                // data-login_uri="http://localhost:8000/api/login"
                data-auto_prompt="false">
            </div>
            <div class="g_id_signin"
                data-type="standard"
                data-shape="rectangular"
                data-theme="outline"
                data-text="signin"
                data-size="large"
                data-logo_alignment="left">
            </div>
        </>
    );
}