<script lang="ts">
  import Empty from "../components/sections/empty.svelte";
import Profile from "../components/sections/profile.svelte";
</script>

<main>
  <div class="list">
    <Profile/>
    <Empty/>
    <Empty/>
    <Empty/>
    <Empty/>
  </div>
</main>

<style>
  main {
    display: flex;
    justify-content: flex-start; 
    align-items: center; 
    height: 100vh; 
  }

  .list {
    display: flex;
    flex-direction: column;
    max-width: 10%;
    margin: 0;
  }
</style>

