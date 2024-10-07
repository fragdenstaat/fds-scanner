import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';
import { computed, ref } from 'vue';


export type FoiMessage = {
    id: number,
    timestamp: String,
    is_response: String,
    sender: String,
    subject: String,
};

export type FoiRequestId = number

export const useFoiMessagesStore = defineStore('foimessages', () => {
    const messages = ref<FoiMessage[]>([])
    const messageMap = computed(() => new Map(messages.value.map((message) => [message.id, message])))

    const getMessages = async (foirequestId: FoiRequestId): Promise<void> => {
        messages.value = await invoke<FoiMessage[]>('get_foimessages', { foirequest_id: foirequestId });
    };

    const clearMessages = () => {
        messages.value = []
    }

    return { messages, messageMap, getMessages, clearMessages }
})