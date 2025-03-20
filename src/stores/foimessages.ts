import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';
import { computed, ref } from 'vue';
import { toLocaleDateString } from '../utils';


type FoiMessageApi = {
    id: number,
    resource_uri: String,
    request: String,
    timestamp: string,
    is_response: boolean,
    is_draft: boolean,
    sender: string | null,
    subject: string,
};

export type FoiMessage = FoiMessageApi & {
    request_id: number;
    timestamp_date: Date;
    timestamp_label: string;
    path: string;
};

export type CreateMessage = {
    request: String;
    timestamp: string;
    kind: "post";
    is_response: boolean;
    sender_public_body: string | null;
    recipient_public_body: string | null;
}

const makeFoiMessage = (mes: FoiMessageApi): FoiMessage => {
    let request_id = mes.request.split('/')[mes.request.split('/').length - 2]
    return {
        ...mes,
        path: `/message/${mes.id}/`,
        request_id: parseInt(request_id),
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

    const getMessage = async (messageId: number): Promise<FoiMessage> => {
        return await _getMessage(messageId)
    }

    const _getMessage = async (messageId: number): Promise<FoiMessage> => {
        if (messageMap.value.has(messageId)) {
            return messageMap.value.get(messageId)!
        }
        try {
            const apiMessage = await invoke<FoiMessageApi>("get_foimessage", { foimessage_id: messageId });
            const message = makeFoiMessage(apiMessage);
            messages.value.push(message);
            return message
        } catch (error) {
            console.error('Error getting message!', error)
            throw error
        }
    }

    const clearMessages = () => {
        messages.value = []
    }

    const createMessage = async (message: CreateMessage): Promise<FoiMessage> => {
        try {
            const apiMessage = await invoke<FoiMessageApi>("create_foimessage", { message });
            const newMessage = makeFoiMessage(apiMessage);
            messages.value = [
                newMessage,
                ...messages.value
            ]
            return newMessage
        } catch (error) {
            console.error('Error creating message!', error)
            throw error
        }
    }

    return { messages, getMessage, getMessages, clearMessages, createMessage }
})