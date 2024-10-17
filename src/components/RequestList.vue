<template>
    <ion-page>
        <ion-header>
            <ion-toolbar>
                <ion-title>Anfragen</ion-title>
                <ion-buttons slot="end">
                    <ion-button :router-link="'/account'">
                        Konto
                    </ion-button>
                </ion-buttons>
            </ion-toolbar>
            <ion-toolbar>
                <ion-searchbar @ion-change="runSearch" @ionInput="searchQuery = $event.target.value || ''"
                    show-clear-button="focus" value="" placeholder="Anfrage suchen"></ion-searchbar>
            </ion-toolbar>
        </ion-header>
        <ion-content class="ion-padding">
            <ion-refresher slot="fixed" @ionRefresh="handleRefresh($event)">
                <ion-refresher-content></ion-refresher-content>
            </ion-refresher>
            <ion-list>
                <TransitionGroup name="list">
                    <ion-item v-for="request in filteredRequests" :router-link="'/request/' + request.id"
                        :key="request.id" router-direction="forward" class="list-item"
                        :class="{ 'staff-request': request.staffRequest }">
                        <ion-label>
                            <h2>
                                {{ request.title }}
                            </h2>
                            <p>an {{ request.public_body.name }} [#{{ request.id }}]</p>
                            <p>
                                {{ request.created_at_label }}<template
                                    v-if="request.last_message_label != request.created_at_label">,
                                    letzte Nachricht vom {{ request.last_message_label }}</template>

                            </p>
                        </ion-label>
                    </ion-item>
                </TransitionGroup>
            </ion-list>
            <div v-if="loading" class="ion-text-center">
                <ion-spinner></ion-spinner>
            </div>
        </ion-content>
    </ion-page>
</template>

<script setup lang="ts">


import { IonButton, IonButtons, IonContent, IonHeader, IonItem, IonLabel, IonList, IonPage, IonRefresher, IonRefresherContent, IonSearchbar, IonSpinner, IonTitle, IonToolbar } from '@ionic/vue';
import { computed, onMounted, ref } from 'vue';

import { account } from '../account.ts';
import { useFoiRequestsStore } from '../stores/foirequests.ts';
import { useToastMessages } from '../utils.ts';

const loading = ref<boolean>(true);
const searchQuery = ref<string>('');
useToastMessages();

const store = useFoiRequestsStore()

const filteredRequests = computed(() => {
    return store.requests.filter((request) => {
        if (searchQuery.value === '') {
            return true;
        }
        return request.title.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
            request.id.toString().startsWith(searchQuery.value);
    });
})

function runSearch() {
    if (account.isStaff) {
        if (/^\d+$/.test(searchQuery.value)) {
            store.getRequest(parseInt(searchQuery.value));
        }
    }
}


onMounted(async () => {
    await store.getRequests();
    loading.value = false;
});


async function handleRefresh(event: CustomEvent) {
    await store.getRequests();
    event.target?.complete();
}
</script>

<style>
h2 small {
    color: var(--ion-color-step-400, var(--ion-text-color-step-600, #999999));
}

.staff-request {
    border-left: 1px solid #999;
}

.list-item {
    z-index: 2;
}

/* 1. declare transition */
.list-move,
.list-enter-active,
.list-leave-active {
    transition: all 0.5s cubic-bezier(0.55, 0, 0.1, 1);
}

/* 2. declare enter from and leave to state */
.list-enter-from,
.list-leave-to {
    opacity: 0;
}

/* 3. ensure leaving items are taken out of layout flow so that moving
      animations can be calculated correctly. */
.list-leave-active {
    position: absolute;
    z-index: 1;
}
</style>