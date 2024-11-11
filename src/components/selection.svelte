<script>
    let showDiv = false;

    function toggleDiv() {
        showDiv = !showDiv;
    }
</script>

<button onclick={toggleDiv}>
    
    <h1>1</h1>
</button>

{#if showDiv}
    <div>
        This is a toggled div.
    </div>
{/if}

<style>
    button {
        width: 100px;
        height: 100px;
        margin: 10px;
        transition: background-color 0.3s;
        border-radius: 100%;
        border: none;
    }

    button:hover {
        background-color: rgb(200, 230, 240);
    }
</style>