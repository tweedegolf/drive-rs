<script lang="ts">
    import type {Crate} from "../crate-db";

    export let crates: Crate[];

    export let filter: number[];

    let filtered_crates: Crate[] = crates;
</script>


<table>
    <thead>
    <tr>
        <th>name {filter[0]}</th>
        <th>description</th>
        <th>downloads</th>
        <th>last updated</th>
    </tr>
    </thead>
    <tbody>
    {#each filtered_crates as crate, i}
        {#if filter.includes(i)}
            <tr>
                <td>{crate.name}</td>
                <td>{crate.description}</td>
                <td>{crate.downloads}</td>
                <td>{crate.updated_at}</td>
            </tr>
        {/if}

    {/each}
    </tbody>
</table>

<style>
    table {
        width: 100%;
        border-collapse: collapse;
    }

    th, td {
        border: 1px solid;
        padding: 0.5em;
    }

    td {
        text-align: left;
    }
</style>
