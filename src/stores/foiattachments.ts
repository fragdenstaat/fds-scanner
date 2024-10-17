import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';
import { computed, ref } from 'vue';


type FoiAttachmentApi = {
    id: number,
    name: string,
    filetype: string,
    size: number,
};

export type FoiAttachment = FoiAttachmentApi & {
    filetype_label: string
    size_label: string
}

const makeFoiAttachment = (att: FoiAttachmentApi): FoiAttachment => {
    let filetype_label = att.filetype
    if (att.filetype === 'application/pdf') {
        filetype_label = 'PDF-Dokument'
    } else if (att.filetype.startsWith('image/')) {
        filetype_label = 'Bild'
    }
    return {
        ...att,
        filetype_label: filetype_label,
        size_label: (att.size / 1024).toFixed(2) + ' KB',
    }
}
export type FoiMessageId = number

export const useFoiAttachmentsStore = defineStore('foiattachments', () => {
    const attachments = ref<FoiAttachment[]>([])
    const attachmentMap = computed(() => new Map(attachments.value.map((att) => [att.id, att])))

    const getAttachments = async (foimessage_id: FoiMessageId): Promise<void> => {
        attachments.value = (await invoke<FoiAttachmentApi[]>('get_foiattachments', { foimessage_id })).map(att => makeFoiAttachment(att));
    };

    const clearAttachments = () => {
        attachments.value = []
    }

    return { attachments, attachmentMap, getAttachments, clearAttachments }
})