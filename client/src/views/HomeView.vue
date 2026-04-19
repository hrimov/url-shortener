<script setup lang="ts">
import { ref, onBeforeUnmount } from 'vue'
import UrlShortenerForm from '@/components/UrlShortenerForm.vue'
import UrlResult from '@/components/UrlResult.vue'
import RecentUrls from '@/components/RecentUrls.vue'
import { SUCCESS_MESSAGE_DURATION_MS } from '@/constants'

const createdShortUrl = ref<string | null>(null)
let successTimeoutId: ReturnType<typeof setTimeout> | null = null

function handleSuccess(shortUrl: string) {
  createdShortUrl.value = shortUrl

  if (successTimeoutId !== null) {
    clearTimeout(successTimeoutId)
  }

  successTimeoutId = setTimeout(() => {
    createdShortUrl.value = null
    successTimeoutId = null
  }, SUCCESS_MESSAGE_DURATION_MS)
}

onBeforeUnmount(() => {
  if (successTimeoutId !== null) {
    clearTimeout(successTimeoutId)
  }
})
</script>

<template>
  <div class="min-h-screen bg-gray-50 px-4 py-8 text-gray-900 sm:py-6">
    <header class="mb-12 text-center">
      <h1
        class="mb-2 bg-gradient-to-br from-blue-500 to-violet-500 bg-clip-text text-4xl font-bold text-transparent sm:text-3xl"
      >
        URL Shortener
      </h1>
      <p class="text-lg text-gray-500 sm:text-base">
        Transform long URLs into short, shareable links
      </p>
    </header>

    <main class="mx-auto max-w-5xl">
      <UrlShortenerForm @success="handleSuccess" />

      <UrlResult v-if="createdShortUrl" :short-url="createdShortUrl" />

      <RecentUrls />
    </main>
  </div>
</template>
