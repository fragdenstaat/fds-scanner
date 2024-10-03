<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-title>Anfragen</ion-title>
            </ion-toolbar>
        </ion-header>
        <ion-content class="ion-padding">
            <ion-searchbar show-clear-button="focus" value="Show on Focus"></ion-searchbar>
            <ion-refresher slot="fixed" @ionRefresh="handleRefresh($event)">
                <ion-refresher-content></ion-refresher-content>
            </ion-refresher>
            <ion-list>
                <ion-item v-for="request in requests" :router-link="'/request/' + request.id"
                    router-direction="forward">
                    <ion-label>
                        <h2>{{ request.title }}</h2>
                        <p>Paragraph</p>
                    </ion-label>
                </ion-item>
            </ion-list>

        </ion-content>
    </ion-page>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/core';

import { IonButton, IonContent, IonHeader, IonItem, IonLabel, IonList, IonPage, IonRefresher, IonRefresherContent, IonSearchbar, IonSkeletonText, IonTitle, IonToolbar } from '@ionic/vue';
import { defineComponent, onMounted, ref } from 'vue';

type Request = {
    id: number;
    title: string;
    date: string;
};

const getRequests = async (): Promise<Request[]> => {
    return await invoke('get_foirequests');
};

export default defineComponent({
    components: {
        IonButton,
        IonContent,
        IonHeader,
        IonItem, IonList, IonLabel,
        IonPage,
        IonRefresher,
        IonRefresherContent,
        IonSkeletonText,
        IonSearchbar,
        IonTitle,
        IonToolbar,
    },
    setup() {
        const loading = ref<boolean>(true);
        const requests = ref<Request[]>([]);

        onMounted(async () => {
            const requestsData = await getRequests();
            requests.value = requestsData;
            loading.value = false;
        });

        async function handleRefresh(event: CustomEvent) {
            const requestsData = await getRequests();
            requests.value = requestsData;
            event.target?.complete();
        }

        return {
            loading,
            requests,
            handleRefresh
        };
    },
});
</script>