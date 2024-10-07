<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-title>Login</ion-title>
            </ion-toolbar>
        </ion-header>
        <ion-content class="ion-padding">
            <template v-if="loginStarted">
                <ion-loading message="Login gestartet..."></ion-loading>
            </template>
            <div v-if="!loginStarted" class="ion-text-center">
                <ion-button @click="startLogin">Einloggen mit FragDenStaat.de</ion-button>
                <hr />
                <p>Zeigt FragDenStaat.de Ihnen einen QR Code an?</p>
                <ion-button :router-link="'/login/qrcode'">Login mit QR Code</ion-button>
            </div>
            <ion-toast v-for="message in toastMessages" :message="message" :duration="5000"></ion-toast>
        </ion-content>
    </ion-page>
</template>

<script setup lang="ts">

import { alertController, IonButton, IonContent, IonHeader, IonLoading, IonPage, IonTitle, IonToast, IonToolbar, useIonRouter } from '@ionic/vue';
import { ref } from 'vue';

import { account } from '../account.ts';

import { useToastMessages } from '../utils.ts';

const { toastMessages } = useToastMessages();

const ionRouter = useIonRouter();
let loginStarted = ref(false);

async function startLogin() {
    loginStarted.value = true;
    console.log("Starting login process");

    let result = await account.startLogin();
    if (result === null) {
        console.log("Login result", result);
        loginStarted.value = true;
        ionRouter.navigate('/', 'none', 'pop');
    } else {
        console.error("Login failed", result);
        loginStarted.value = false;
        const alert = await alertController.create({
            header: 'Login fehlgeschlagen',
            subHeader: 'Folgende Details wurden übermittelt',
            message: result,
            buttons: ['Schade'],
        });
        await alert.present();
    }
}

</script>
