<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-title>Login</ion-title>
            </ion-toolbar>
        </ion-header>
        <ion-content class="ion-padding">
            <ion-grid>
                <ion-row class="ion-justify-content-center">
                    <ion-col size-xs="9" size-md="9">
                        <template v-if="loginStarted">
                            <ion-loading message="Login gestartet..."></ion-loading>
                        </template>
                        <div v-if="!loginStarted" class="ion-text-center">
                            <ion-button @click="startLogin">Einloggen mit FragDenStaat.de</ion-button>
                            <hr class="ion-margin-vertical" />
                            <p>Zeigt FragDenStaat.de Ihnen einen QR Code an?</p>

                            <ion-button :router-link="'/login/qrcode'">
                                <ion-icon aria-hidden="true" :icon="qrCodeOutline" class="ion-margin-end"></ion-icon>
                                Login mit QR Code
                            </ion-button>
                        </div>
                    </ion-col>
                </ion-row>
            </ion-grid>
        </ion-content>
    </ion-page>
</template>

<script setup lang="ts">

import { alertController, IonButton, IonCol, IonContent, IonGrid, IonHeader, IonIcon, IonLoading, IonPage, IonRow, IonTitle, IonToolbar, useIonRouter } from '@ionic/vue';
import { qrCodeOutline } from 'ionicons/icons';
import { onMounted, ref } from 'vue';

import { account } from '../account.ts';

import { useToastMessages } from '../utils.ts';

useToastMessages();

const ionRouter = useIonRouter();
let loginStarted = ref(false);

onMounted(() => {
    if (account.startLoginOnMount()) {
        startLogin();
    }
});

async function startLogin() {
    loginStarted.value = true;
    console.log("Starting login process");

    let result = await account.startLogin();
    if (result === null) {
        console.log("Login result", result);
        loginStarted.value = true;
        let nextPath = account.getNextPath();
        ionRouter.navigate(nextPath, 'none', 'pop');
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

<style>
ion-grid {
    height: 100%;
}

ion-row {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
}
</style>