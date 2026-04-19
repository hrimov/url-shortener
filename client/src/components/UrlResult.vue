<script setup lang="ts">
import { ref, computed } from 'vue'
import { api } from '@/services/api'
import { COPY_SUCCESS_DURATION_MS } from '@/constants'

const props = defineProps<{
  shortUrl: string
}>()

const copied = ref(false)

const fullUrl = computed(() => api.getShortUrl(props.shortUrl))

async function copyToClipboard() {
  try {
    await navigator.clipboard.writeText(fullUrl.value)
    copied.value = true
    setTimeout(() => {
      copied.value = false
    }, COPY_SUCCESS_DURATION_MS)
  } catch (err) {
    console.error('Failed to copy:', err)
  }
}
</script>

<template>
  <div
    class="mx-auto mt-8 max-w-xl rounded-xl border border-green-300 bg-green-50 p-8 text-center"
    role="status"
    aria-live="polite"
  >
    <div class="mx-auto mb-4 flex size-12 items-center justify-center rounded-full bg-green-500 text-white">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="2"
        stroke="currentColor"
        class="size-6"
        aria-hidden="true"
      >
        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
      </svg>
    </div>

    <h3 class="mb-6 text-xl font-semibold text-green-800">Your short URL is ready!</h3>

    <div class="mb-6 rounded-lg border border-gray-300 bg-white p-4">
      <a
        :href="fullUrl"
        target="_blank"
        rel="noopener noreferrer"
        class="break-all text-lg font-medium text-blue-500 hover:underline"
      >
        {{ fullUrl }}
      </a>
    </div>

    <div class="flex justify-center gap-3">
      <button
        @click="copyToClipboard"
        :aria-label="copied ? 'Copied to clipboard' : 'Copy short URL to clipboard'"
        class="cursor-pointer rounded-lg bg-green-500 px-6 py-3 text-sm font-semibold text-white transition-all hover:bg-green-600"
      >
        {{ copied ? 'Copied!' : 'Copy URL' }}
      </button>
    </div>
  </div>
</template>
