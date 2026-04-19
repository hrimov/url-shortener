<script setup lang="ts">
import { ref, computed } from 'vue'
import { useUrlStore } from '@/stores/url'
import { CUSTOM_SHORT_URL_MIN_LENGTH, CUSTOM_SHORT_URL_MAX_LENGTH, BASE62_PATTERN } from '@/constants'
import { normalizeUrl, isValidUrl } from '@/utils/url'

const urlStore = useUrlStore()

const longUrl = ref('')
const customShortUrl = ref('')
const useCustom = ref(false)

const emit = defineEmits<{
  success: [shortUrl: string]
}>()

const longUrlError = computed(() => {
  const raw = longUrl.value.trim()
  if (!raw) return null
  return isValidUrl(raw) ? null : 'Please enter a valid URL'
})

const customUrlError = computed(() => {
  if (!useCustom.value || !customShortUrl.value) return null

  const trimmed = customShortUrl.value.trim()
  if (trimmed.length < CUSTOM_SHORT_URL_MIN_LENGTH || trimmed.length > CUSTOM_SHORT_URL_MAX_LENGTH) {
    return `Must be ${CUSTOM_SHORT_URL_MIN_LENGTH}-${CUSTOM_SHORT_URL_MAX_LENGTH} characters`
  }
  if (!BASE62_PATTERN.test(trimmed)) {
    return 'Only alphanumeric characters allowed (0-9, a-z, A-Z)'
  }
  return null
})

async function handleSubmit() {
  const raw = longUrl.value.trim()
  if (!raw || longUrlError.value || customUrlError.value) {
    return
  }

  const result = await urlStore.createShortUrl({
    long_url: normalizeUrl(raw),
    custom_short_url: useCustom.value && customShortUrl.value ? customShortUrl.value.trim() : undefined,
  })

  if (result) {
    emit('success', result.short_url)
    longUrl.value = ''
    customShortUrl.value = ''
    useCustom.value = false
  }
}
</script>

<template>
  <form @submit.prevent="handleSubmit" class="mx-auto flex max-w-xl flex-col gap-6">
    <div class="flex flex-col gap-2">
      <label for="long-url" class="text-sm font-semibold text-gray-700">Long URL</label>
      <input
        id="long-url"
        v-model="longUrl"
        type="text"
        placeholder="https://example.com/very/long/url or google.com"
        required
        :aria-invalid="!!longUrlError"
        :aria-describedby="longUrlError ? 'long-url-error' : undefined"
        class="rounded-lg border border-gray-300 px-4 py-3 text-base transition-all focus:border-blue-500 focus:ring-3 focus:ring-blue-500/10 focus:outline-none disabled:cursor-not-allowed disabled:bg-gray-100"
        :disabled="urlStore.loading"
      />
      <p v-if="longUrlError" id="long-url-error" role="alert" class="text-xs text-red-700">
        {{ longUrlError }}
      </p>
    </div>

    <div class="flex items-center">
      <label class="flex cursor-pointer items-center gap-2 select-none">
        <input
          type="checkbox"
          v-model="useCustom"
          :disabled="urlStore.loading"
          class="size-4.5 cursor-pointer"
        />
        <span>Use custom short URL</span>
      </label>
    </div>

    <div v-if="useCustom" class="flex flex-col gap-2">
      <label for="custom-url" class="text-sm font-semibold text-gray-700">Custom short URL</label>
      <input
        id="custom-url"
        v-model="customShortUrl"
        type="text"
        placeholder="myCustomLink"
        :aria-invalid="!!customUrlError"
        :aria-describedby="customUrlError ? 'custom-url-error' : 'custom-url-hint'"
        class="rounded-lg border px-4 py-3 text-base transition-all focus:outline-none disabled:cursor-not-allowed disabled:bg-gray-100"
        :class="
          customUrlError
            ? 'border-red-500 focus:border-red-600 focus:ring-3 focus:ring-red-500/10'
            : 'border-gray-300 focus:border-blue-500 focus:ring-3 focus:ring-blue-500/10'
        "
        :disabled="urlStore.loading"
      />
      <p v-if="customUrlError" id="custom-url-error" role="alert" class="text-xs text-red-700">
        {{ customUrlError }}
      </p>
      <p v-else id="custom-url-hint" class="text-xs text-gray-500">
        Only alphanumeric characters ({{ CUSTOM_SHORT_URL_MIN_LENGTH }}-{{ CUSTOM_SHORT_URL_MAX_LENGTH }} chars)
      </p>
    </div>

    <button
      type="submit"
      :aria-busy="urlStore.loading"
      class="rounded-lg bg-blue-500 px-6 py-3.5 text-base font-semibold text-white transition-all hover:bg-blue-600 disabled:cursor-not-allowed disabled:bg-gray-400"
      :disabled="urlStore.loading || !longUrl.trim() || !!longUrlError || !!customUrlError"
    >
      {{ urlStore.loading ? 'Creating...' : 'Shorten URL' }}
    </button>

    <div
      v-if="urlStore.error"
      class="rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700"
      role="alert"
    >
      {{ urlStore.error }}
    </div>
  </form>
</template>
