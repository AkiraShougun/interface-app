<script lang="ts">
  import Selection from "../components/selection.svelte";
</script>

<main>
  <div class="selection">
    <Selection/>
    <Selection/>
    <Selection/>
    <Selection/>
  </div>
</main>

<style>
  .selection {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    justify-content: center;
    height: 100vh;
    padding-left: 20px;
  }
</style>

