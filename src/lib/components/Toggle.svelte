<script lang="ts">
  export let checked = false;
</script>

<button
  class="flex items-center cursor-pointer relative mb-4"
  on:click
>
  <div
    class="{checked
      ? 'bg-blue-600 border-blue-600'
      : ''} bg-gray-200 border-2 border-gray-200 h-6 w-11 rounded-full hover:cursor-pointer"
  />
  <div
    class="{checked
      ? 'translate-x-full border-white'
      : ''} absolute top-0.5 left-0.5  
    bg-white border border-gray-300 rounded-full 
    h-5 w-5 transition shadow-sm"
  />
</button>
