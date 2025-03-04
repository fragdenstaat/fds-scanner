<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-buttons slot="start">
                    <ion-back-button default-href="/" text="Zurück"></ion-back-button>
                </ion-buttons>
                <ion-title>Info</ion-title>
            </ion-toolbar>
        </ion-header>
        <ion-content class="ion-padding">
            <h2>Über diese App</h2>
            <p>FDS Scanner ist eine App von FragDenStaat, um das Scannen und Hochladen von Post, die im Rahmen von
                Anfragen nach den Informationsfreiheitsgesetzen empfangen oder gesendet wurde zu vereinfachen.</p>

            <p>FragDenStaat ist ein Projekt des Open Knowledge Foundation Deutschland e.V.</p>

            <h2>Impressum</h2>
            <p>Open Knowledge Foundation Deutschland e.V.<br>
                FragDenStaat<br>
                Singerstr. 109<br>
                10179 Berlin<br>
                <a href="https://okfn.de/" target="_blank">https://okfn.de</a><br>
                Fax: +49-30-85102320
            </p>

            <p>Eingetragen beim Amtsgericht Charlottenburg, VR 30468 B.</p>

            <p>Vertretungsberechtigter Vorstand: Kristina Klein, Felix Reda, Lea Gimpel, Gabriele C. Klug, Stefan
                Heumann, Elina Eickstädt, Fiona Krakenbürger.</p>

            <h2>Datenschutz</h2>

            <p>
                <a href="https://fragdenstaat.de/datenschutzerklaerung/" target="_blank">Datenschutzerklärung</a>
            </p>
        </ion-content>
    </ion-page>
</template>

<script setup lang="ts">

import { alertController, IonBackButton, IonButtons, IonContent, IonHeader, IonPage, IonTitle, IonToolbar, useIonRouter } from '@ionic/vue';
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