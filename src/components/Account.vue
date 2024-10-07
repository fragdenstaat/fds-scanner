<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-buttons slot="start">
                    <ion-back-button text="Zurück"></ion-back-button>
                </ion-buttons>
                <ion-title>Konto</ion-title>
            </ion-toolbar>
        </ion-header>
        <ion-content class="ion-padding">
            <template v-if="logoutStarted">
                <ion-loading message="Sie werde ausgeloggt"></ion-loading>
            </template>
            <template v-else>
                <h2>Hallo {{ account.user?.first_name }}!</h2>
                <p>
                    Sie sind eingeloggt als {{ account.user?.email }}.
                </p>
                <ion-button @click="startLogout">Ausloggen</ion-button>
            </template>
        </ion-content>
    </ion-page>
</template>

<script setup lang="ts">

import { alertController, IonBackButton, IonButton, IonButtons, IonContent, IonHeader, IonLoading, IonPage, IonTitle, IonToolbar, useIonRouter } from '@ionic/vue';
import { ref } from 'vue';

import { account } from '../account.ts';

const ionRouter = useIonRouter();
let logoutStarted = ref(false);

async function startLogout() {
    logoutStarted.value = true;
    console.log("Starting logout process");
    let result = await account.startLogout();
    if (result === null) {
        logoutStarted.value = true;
        ionRouter.navigate('/login', 'root', 'pop');

    } else {
        console.error("Logout failed", result);
        logoutStarted.value = false;
        const alert = await alertController.create({
            header: 'Logout fehlgeschlagen',
            subHeader: 'Folgende Details wurden übermittelt',
            message: result,
            buttons: ['Schade'],
        });
        await alert.present();

    }
}
</script>