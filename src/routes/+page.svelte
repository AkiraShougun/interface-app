<script lang="ts">
  import Empty from "../components/sections/empty.svelte";
import Profile from "../components/sections/profile.svelte";
</script>

<main>
  <div class="list">
    <Profile/>
    <Empty text="1"/>
    <Empty text="2"/>
    <Empty text="3"/>
    <Empty text="4"/>
  </div>
</main>

<style>
  main {
    display: flex;
    justify-content: flex-start; 
    align-items: center; 
    height: 100vh; 
    color:white;
  }

  .list {
    display: flex;
    flex-direction: column;
    max-width: 10%;
    margin: 0 auto;
    transform: translateX(-130%);
  }
</style>

