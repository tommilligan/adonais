/*
 * Copyright 2016 Google Inc. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
 * in compliance with the License. You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 * express or implied. See the License for the specific language governing permissions and
 * limitations under the License.
 */

/**
 * FirebaseUI initialization to be used in a Single Page application context.
 */

/**
 * @return {!Object} The FirebaseUI config.
 */
function getUiConfig() {
    return {
        callbacks: {
            // Called when the user has been successfully signed in.
            signInSuccessWithAuthResult: function(authResult, redirectUrl) {
                if (authResult.user) {
                    handleSignedInUser(authResult.user);
                }
                if (authResult.additionalUserInfo) {
                    document.getElementById(
                        "is-new-user"
                    ).textContent = authResult.additionalUserInfo.isNewUser
                        ? "New User"
                        : "Existing User";
                }
                // Do not redirect.
                return false;
            }
        },
        // Opens IDP Providers sign-in flow in a popup.
        signInFlow: "redirect",
        signInOptions: [
            {
                provider: firebase.auth.GoogleAuthProvider.PROVIDER_ID,
                scopes: config.scopes,
                // Required to enable this provider in One-Tap Sign-up.
                authMethod: "https://accounts.google.com",
                // Required to enable ID token credentials for this provider.
                clientId: config.clientId
            }
        ],
        // Terms of service url.
        tosUrl: "https://www.google.com",
        // Privacy policy url.
        privacyPolicyUrl: "https://www.google.com",
        credentialHelper: firebaseui.auth.CredentialHelper.ACCOUNT_CHOOSER_COM
    };
}

// Initialize the FirebaseUI Widget using Firebase.
var ui = new firebaseui.auth.AuthUI(firebase.auth());
// Disable auto-sign in.
ui.disableAutoSignIn();

const URI =
    "https://lsm-education.kcl.ac.uk/apicommonstring/api/values/Mod-Module.5MBBSStage2";

async function fetchKeatsData() {
    const response = await fetch(URI);
    return response.json();
}

async function insertCalendarAndSaveId(user_document_ref) {
    console.log("Inserting new calendar");
    let response = await gapi.client.calendar.calendars.insert({
        summary: "King's (via adonais)",
        timeZone: "Europe/London"
    });
    let calendar_id = response.result.id;
    user_document_ref.set({ calendar_id: calendar_id });
    return calendar_id;
}

async function startApp(user) {
    console.log("Starting app...");
    var user_document_ref = firebase
        .firestore()
        .collection("users")
        .doc(user.user.uid);

    var user_document = await user_document_ref.get();
    var existing_calendar_id = user_document.get("calendar_id");

    // if we don't have an existing calendar id, create one and get the id
    // if we do have one, try to load it
    // if it turns out not to exist, the user deleted it, just create another

    if (existing_calendar_id) {
        console.log("Already created calendar " + existing_calendar_id);
        try {
            await gapi.client.calendar.calendars.get({
                calendarId: existing_calendar_id
            });
        } catch (error) {
            console.warn(error);
            if (error.status === 404) {
                console.log("Missing calendar, create again");
                existing_calendar_id = await insertCalendarAndSaveId(
                    user_document_ref
                );
            } else {
                throw error;
            }
        }
    } else {
        existing_calendar_id = insertCalendarAndSaveId(user_document_ref);
    }

    console.log(existing_calendar_id);
}

/**
 * Displays the UI for a signed in user.
 * @param {!firebase.User} user
 */
var handleSignedInUser = function(user) {
    function start() {
        // 2. Initialize the JavaScript client library.
        gapi.client
            .init({
                apiKey: config.apiKey,
                clientId: config.clientId,
                discoveryDocs: config.discoveryDocs,
                scope: config.scopes.join(" ")
            })
            .then(function() {
                var googleUser = gapi.auth2.getAuthInstance().currentUser.get();
                var isSignedIn = gapi.auth2.getAuthInstance().isSignedIn.get();
                if (isSignedIn) {
                    // get the credentials from the google auth response
                    var idToken = googleUser.getAuthResponse().id_token;
                    var creds = firebase.auth.GoogleAuthProvider.credential(
                        idToken
                    );
                    // auth in the user
                    firebase
                        .auth()
                        .signInWithCredential(creds)
                        .then(user => {
                            // you can use (user) or googleProfile to setup the user
                            var googleProfile = googleUser.getBasicProfile();

                            var syncCalendar = function() {
                                startApp(user);
                            };
                            document
                                .getElementById("sync-calendar")
                                .addEventListener("click", syncCalendar);
                        });
                }
            });
    }
    // 1. Load the JavaScript client library.
    gapi.load("client", start);

    // Add to the document
    document.getElementById("user-signed-in").style.display = "block";
    document.getElementById("user-signed-out").style.display = "none";
    document.getElementById("name").textContent = user.displayName;
    document.getElementById("email").textContent = user.email;
    document.getElementById("phone").textContent = user.phoneNumber;
    if (user.photoURL) {
        var photoURL = user.photoURL;
        // Append size to the photo URL for Google hosted images to avoid requesting
        // the image with its original resolution (using more bandwidth than needed)
        // when it is going to be presented in smaller size.
        if (
            photoURL.indexOf("googleusercontent.com") != -1 ||
            photoURL.indexOf("ggpht.com") != -1
        ) {
            photoURL =
                photoURL +
                "?sz=" +
                document.getElementById("photo").clientHeight;
        }
        document.getElementById("photo").src = photoURL;
        document.getElementById("photo").style.display = "block";
    } else {
        document.getElementById("photo").style.display = "none";
    }
};

/**
 * Displays the UI for a signed out user.
 */
var handleSignedOutUser = function() {
    document.getElementById("user-signed-in").style.display = "none";
    document.getElementById("user-signed-out").style.display = "block";
    ui.start("#firebaseui-container", getUiConfig());
};

// Listen to change in auth state so it displays the correct UI for when
// the user is signed in or not.
firebase.auth().onAuthStateChanged(function(user) {
    document.getElementById("loading").style.display = "none";
    document.getElementById("loaded").style.display = "block";
    user ? handleSignedInUser(user) : handleSignedOutUser();
});

/**
 * Deletes the user's account.
 */
var deleteAccount = function() {
    firebase
        .auth()
        .currentUser.delete()
        .catch(function(error) {
            if (error.code == "auth/requires-recent-login") {
                // The user's credential is too old. She needs to sign in again.
                firebase
                    .auth()
                    .signOut()
                    .then(function() {
                        // The timeout allows the message to be displayed after the UI has
                        // changed to the signed out state.
                        setTimeout(function() {
                            alert(
                                "Please sign in again to delete your account."
                            );
                        }, 1);
                    });
            }
        });
};

/**
 * Initializes the app.
 */
var initApp = function() {
    document.getElementById("sign-out").addEventListener("click", function() {
        firebase.auth().signOut();
    });
    document
        .getElementById("delete-account")
        .addEventListener("click", function() {
            deleteAccount();
        });
};

window.addEventListener("load", initApp);
