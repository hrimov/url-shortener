<script setup lang="ts">
import { computed } from 'vue'
import { useUrlStore } from '@/stores/url'
import { api } from '@/services/api'
import { DATE_FORMAT_OPTIONS } from '@/constants'

const urlStore = useUrlStore()

const hasRecentUrls = computed(() => urlStore.recentUrls.length > 0)

function formatDate(dateString: string): string {
  const date = new Date(dateString)
  return new Intl.DateTimeFormat('en-US', DATE_FORMAT_OPTIONS).format(date)
}

function getFullUrl(shortUrl: string): string {
  return api.getShortUrl(shortUrl)
}
</script>

<template>
  <div v-if="hasRecentUrls" class="mx-auto mt-12 max-w-xl">
    <h3 class="mb-4 text-lg font-semibold text-gray-900">Recent URLs</h3>

    <ul class="flex flex-col gap-3" aria-label="Recently shortened URLs">
      <li
        v-for="url in urlStore.recentUrls"
        :key="url.short_url"
        class="flex items-start justify-between gap-4 rounded-lg border border-gray-200 bg-white p-4 transition-all hover:border-gray-300 hover:shadow-sm"
      >
        <div class="min-w-0 flex-1">
          <a
            :href="getFullUrl(url.short_url)"
            target="_blank"
            rel="noopener noreferrer"
            class="break-all text-[0.9375rem] font-semibold text-blue-500 hover:underline"
          >
            {{ url.short_url }}
          </a>
          <p class="mt-1 truncate text-[0.8125rem] text-gray-500" :title="url.long_url">
            {{ url.long_url }}
          </p>
        </div>
        <div class="flex shrink-0 flex-col items-end gap-2">
          <span class="whitespace-nowrap text-xs text-gray-400">
            {{ formatDate(url.created_at) }}
          </span>
          <span
            :class="[
              'rounded px-2 py-0.5 text-[0.6875rem] font-semibold uppercase',
              url.status === 'active'
                ? 'bg-emerald-100 text-emerald-800'
                : 'bg-red-50 text-red-800',
            ]"
          >
            {{ url.status }}
          </span>
        </div>
      </li>
    </ul>

    <button
      @click="urlStore.clearRecentUrls"
      aria-label="Clear URL history"
      class="mt-4 w-full cursor-pointer rounded-md border border-gray-200 bg-transparent px-4 py-2 text-sm text-gray-500 transition-all hover:border-gray-300 hover:bg-gray-50"
    >
      Clear history
    </button>
  </div>
</template>
