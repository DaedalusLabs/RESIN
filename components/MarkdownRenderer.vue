<!-- eslint-disable vue/no-v-html -->
<template>
   <div v-html="renderedContent" />
</template>

<script setup lang="ts">
import { marked } from "marked";
import DOMPurify from "dompurify";

const props = defineProps<{
   content: string;
}>();

// Configure marked to be strict and secure
marked.setOptions({
   gfm: true,
   breaks: true,
   pedantic: false,
});

// Render markdown and sanitize the output
const renderedContent = computed(() => {
   const rawHtml = marked.parse(props.content || "") as string;
   return DOMPurify.sanitize(rawHtml);
});
</script>
