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

import { IonBackButton, IonButtons, IonContent, IonHeader, IonPage, IonTitle, IonToolbar } from '@ionic/vue';

import { useLoggedOutDeepLinkNavigation } from '../account.ts';
import { useToastMessages } from '../utils.ts';

useToastMessages();
useLoggedOutDeepLinkNavigation()

</script>
