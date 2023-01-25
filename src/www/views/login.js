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

// export function LoginButton() {
//     const [loaded, setLoaded] = useState(false);
    
//     const signIn = useCallback(() => {
//         // Requires OAuth2 to be loaded.
//         if (!loaded) {
//             return;
//         }
//         window.gapi.auth2
//             .getAuthInstance()
//             .signIn()
//             .then((res) => {
//                 console.log("Signed-in:", res);
//             })
//             .catch((err) => {
//                 console.error("Sign-in error:", err);
//             });
//     }, [loaded]);
//     // Load Google API and OAuth2
//     useEffect(async () => {
//         await import("https://apis.google.com/js/platform.js");
//         await window.gapi.load("auth2");
//         await new Promise((resolve) => setTimeout(resolve, 50));
//         await window.gapi.auth2.init({ client_id: CLIENT_ID });
        
//         setLoaded(true);
//         if (window.gapi.auth2.getAuthInstance().isSignedIn.get()) {
//             console.log("Signed in:", auth2.currentUser.get());
//         }
//     }, []);

//     return loaded && (
//         <button onclick={signIn}>Login with Google</button>
//     );
// }