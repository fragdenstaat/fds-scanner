<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-title>Login</ion-title>
            </ion-toolbar>
        </ion-header>
        <ion-content class="ion-padding">
            <ion-button router-link="/" router-direction="forward">Click Me</ion-button>
        </ion-content>
    </ion-page>
</template>

<script lang="ts">
import { IonButton, IonContent, IonHeader, IonPage, IonTitle, IonToolbar } from '@ionic/vue';
import { defineComponent } from 'vue';

export default defineComponent({
    components: {
        IonContent,
        IonButton,
        IonHeader,
        IonPage,
        IonTitle,
        IonToolbar,
    },
});
</script>