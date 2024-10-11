import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { defineStore } from 'pinia';
import { computed, ref } from 'vue';
import { toLocaleDateString } from '../utils';

type PublicBodyApi = {
    id: number;
    name: string;
}


type FoiRequestApi = {
    id: number;
    title: string;
    created_at: string;
    last_message: string;
    public_body: PublicBodyApi;
};

export type FoiRequest = FoiRequestApi & {
    created_at_date: Date;
    created_at_label: string;
    last_message_date: Date;
    last_message_label: string;
};


const makeFoiRequest = (req: FoiRequestApi): FoiRequest => {
    return {
        ...req,
        created_at_date: new Date(req.created_at),
        created_at_label: toLocaleDateString(new Date(req.created_at)),
        last_message_date: new Date(req.last_message),
        last_message_label: toLocaleDateString(new Date(req.last_message)),
    }
}

export const useFoiRequestsStore = defineStore('foirequests', () => {

    const getRequests = async (): Promise<boolean> => {
        return await invoke('get_foirequests');
    };

    const requests = ref<FoiRequest[]>([])
    const requestMap = computed(() => new Map(requests.value.map((request) => [request.id, request])))

    listen<FoiRequestApi[]>('foirequest-list', (event) => {
        event.payload.map((request) => {
            if (!requestMap.value.has(request.id)) {
                requests.value.push(makeFoiRequest(request));
            }
        });
    });

    return { requests, requestMap, getRequests }
})