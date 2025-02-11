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
    staffRequest: boolean;
};


const makeFoiRequest = (req: FoiRequestApi, staffRequest: boolean = false): FoiRequest => {
    return {
        ...req,
        created_at_date: new Date(req.created_at),
        created_at_label: toLocaleDateString(new Date(req.created_at)),
        last_message_date: new Date(req.last_message),
        last_message_label: toLocaleDateString(new Date(req.last_message)),
        staffRequest
    }
}

export const useFoiRequestsStore = defineStore('foirequests', () => {

    const getRequests = async (): Promise<boolean> => {
        return await invoke('get_foirequests');
    };

    const addFoirequest = (request: FoiRequestApi): FoiRequest => {
        if (!requestMap.value.has(request.id)) {
            let foirequest = makeFoiRequest(request, staffRequests.has(request.id))
            requests.value.push(foirequest);
            return foirequest
        }
        return requestMap.value.get(request.id)!
    }

    const staffRequests = new Set<number>()

    const getRequest = async (foirequestId: number, isStaff: boolean = false): Promise<FoiRequest> => {
        if (requestMap.value.has(foirequestId)) {
            return requestMap.value.get(foirequestId)!
        }
        if (isStaff) {
            staffRequests.add(foirequestId)
        }
        try {
            let foirequest = await invoke<Promise<FoiRequestApi>>('get_foirequest', { request_id: foirequestId })
            return addFoirequest(foirequest)
        } catch (error) {
            console.error('Error getting request!', error)
            throw error
        }
    }

    const requests = ref<FoiRequest[]>([])
    const requestMap = computed(() => new Map(requests.value.map((request) => [request.id, request])))

    listen<FoiRequestApi[]>('foirequest-list', (event) => {
        event.payload.map((request) => {
            addFoirequest(request)
        });
    });

    return { requests, getRequests, getRequest }
})