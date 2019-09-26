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

var config = {
    apiKey: "AIzaSyAryo9PZIdOfuGvZ07llXn1WkmMpT48uT8",
    authDomain: "adonais-a3bf8.firebaseapp.com",
    databaseURL: "https://adonais-a3bf8.firebaseio.com",
    projectId: "adonais-a3bf8",
    storageBucket: "adonais-a3bf8.appspot.com",
    messagingSenderId: "1049227684135",
    appId: "1:1049227684135:web:f0c7b1cc1586c32f056102",
    measurementId: "G-N92MBV6XP2",
    scopes: [
        "email",
        "profile",
        "https://www.googleapis.com/auth/calendar",
        "https://www.googleapis.com/auth/calendar.events"
    ],
    discoveryDocs: [
        "https://www.googleapis.com/discovery/v1/apis/calendar/v3/rest"
    ],
    // Google OAuth Client ID, needed to support One-tap sign-up.
    // Set to null if One-tap sign-up is not supported.
    clientId:
        "1049227684135-dn2clgk28p5fnn4bmvhi6068m5jkeoa3.apps.googleusercontent.com"
};
firebase.initializeApp(config);
