import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';
import { computed, ref } from 'vue';
import { toLocaleDateString } from '../utils';


type FoiMessageApi = {
    id: number,
    timestamp: string,
    is_response: string,
    sender: string,
    subject: string,
};

export type FoiMessage = FoiMessageApi & {
    timestamp_date: Date;
    timestamp_label: string;
};

const makeFoiMessage = (mes: FoiMessageApi): FoiMessage => {
    return {
        ...mes,
        timestamp_date: new Date(mes.timestamp),
        timestamp_label: toLocaleDateString(new Date(mes.timestamp)),
    }
}

export type FoiRequestId = number

export const useFoiMessagesStore = defineStore('foimessages', () => {
    const messages = ref<FoiMessage[]>([])
    const messageMap = computed(() => new Map(messages.value.map((message) => [message.id, message])))

    const getMessages = async (foirequestId: FoiRequestId): Promise<void> => {
        messages.value = (await invoke<FoiMessageApi[]>('get_foimessages', { foirequest_id: foirequestId })).map(m => makeFoiMessage(m))
    };

    const clearMessages = () => {
        messages.value = []
    }

    return { messages, messageMap, getMessages, clearMessages }
})