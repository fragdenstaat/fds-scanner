import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';
import { computed, ref } from 'vue';


type FoiAttachmentApi = {
    id: number
    name: string
    filetype: string
    size: number
    belongs_to: string
    site_url: string
    file_url: string
};

export type FoiAttachment = FoiAttachmentApi & {
    filetype_label: string
    size_label: string
    path: string
    message_path: string
    message_id: number
    message_is_draft: boolean
}

const makeFoiAttachment = (att: FoiAttachmentApi): FoiAttachment => {
    let filetype_label = att.filetype
    if (att.filetype === 'application/pdf') {
        filetype_label = 'PDF-Dokument'
    } else if (att.filetype.startsWith('image/')) {
        filetype_label = 'Bild'
    }
    const parts = att.belongs_to.split('/')
    const message_id = parts[parts.length - 2]
    const message_type = parts[parts.length - 3]
    return {
        ...att,
        path: `/attachment/${att.id}/`,
        filetype_label: filetype_label,
        size_label: (att.size / 1024).toFixed(2) + ' KB',
        message_path: `/${message_type}/${message_id}/`,
        message_id: parseInt(message_id),
        message_is_draft: message_type === 'draft'
    }
}
export type FoiMessageId = number

export const useFoiAttachmentsStore = defineStore('foiattachments', () => {
    const attachments = ref<FoiAttachment[]>([])
    const attachmentMap = computed(() => new Map(attachments.value.map((att) => [att.id, att])))

    const getAttachments = async (foimessage_id: FoiMessageId): Promise<void> => {
        attachments.value = (await invoke<FoiAttachmentApi[]>('get_foiattachments', { foimessage_id })).map(att => makeFoiAttachment(att));
    };

    const getAttachment = async (attachmentId: number): Promise<FoiAttachment> => {
        if (attachmentMap.value.has(attachmentId)) {
            return attachmentMap.value.get(attachmentId)!
        }
        try {
            const apiAttachment = await invoke<FoiAttachmentApi>('get_foiattachment', { foiattachment_id: attachmentId });
            const attachment = makeFoiAttachment(apiAttachment);
            attachments.value.push(attachment);
            return attachment
        } catch (error) {
            console.error('Error getting attachment!', error)
            throw error
        }
    }


    const clearAttachments = () => {
        attachments.value = []
    }

    return { attachments, attachmentMap, getAttachments, getAttachment, clearAttachments }
})