import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';
import { computed, ref } from 'vue';


export type FoiAttachment = {
    id: number,
    name: String,
    filetype: String,
    size: number,
};

export type FoiMessageId = number

export const useFoiAttachmentsStore = defineStore('foiattachments', () => {
    const attachments = ref<FoiAttachment[]>([])
    const attachmentMap = computed(() => new Map(attachments.value.map((att) => [att.id, att])))

    const getAttachments = async (foimessage_id: FoiMessageId): Promise<void> => {
        attachments.value = await invoke<FoiAttachment[]>('get_foiattachments', { foimessage_id });
    };

    const clearAttachments = () => {
        attachments.value = []
    }

    return { attachments, attachmentMap, getAttachments, clearAttachments }
})