import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { defineStore } from 'pinia';
import { computed, ref } from 'vue';


export type FoiRequest = {
    id: number;
    title: string;
    created_at: string;
    last_message: string;
};

export const useFoiRequestsStore = defineStore('foirequests', () => {

    const getRequests = async (): Promise<boolean> => {
        return await invoke('get_foirequests');
    };

    const requests = ref<FoiRequest[]>([])
    const requestMap = computed(() => new Map(requests.value.map((request) => [request.id, request])))

    listen<FoiRequest[]>('foirequest-list', (event) => {
        event.payload.map((request) => {
            if (!requestMap.value.has(request.id)) {
                requests.value.push(request);
            }
        });
    });

    return { requests, requestMap, getRequests }
})