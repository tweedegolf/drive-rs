<script lang="ts">
    export let selected: string[];
    export let options: string[];

    let filter: string = "";
</script>

<fieldset>
    <legend>Columns</legend>
    <input type="text" bind:value={filter}/>
    <div class="list">
        {#each options as value}
            {#if value.toLowerCase().includes(filter.toLowerCase())}
                <label>
                    <input type="checkbox" bind:group="{selected}" value={value}>
                    {value}
                </label>
            {/if}
        {/each}
    </div>
</fieldset>
