<script lang="ts">
    import type {Crate} from "../crate-db";

    export let crates: Crate[];

    export let filter: number[];
    export let cols_shown: string[] = ["name", "description", "downloads", "updated_at"];

    let filtered_crates: Crate[] = crates;
</script>

<table>
    <thead>
    <tr>
        <th>Crate</th>
        {#if cols_shown.includes("description")}
            <th>Description</th>
        {/if}
        {#if cols_shown.includes("interfaces")}
            <th>Interfaces</th>
        {/if}
        {#if cols_shown.includes("license")}
            <th>License</th>
        {/if}
        {#if cols_shown.includes("downloads")}
            <th>Downloads</th>
        {/if}
        {#if cols_shown.includes("updated_at")}
            <th>Last Updated</th>
        {/if}
        {#if cols_shown.includes("created_at")}
            <th>Created At</th>
        {/if}
        {#if cols_shown.includes("rust_version")}
            <th>Rust Version</th>
        {/if}
    </tr>
    </thead>
    <tbody>
    {#each filtered_crates as crate, i}
        {#if filter.includes(i)}
            <tr>
                <td><a href="https://crates.io/crates/{crate.name}">{crate.name}
                    {#if cols_shown.includes("version")}@ {crate.version}{/if}
                </a></td>
                {#if cols_shown.includes("description")}
                    <td>{crate.description}</td>
                {/if}
                {#if cols_shown.includes("interfaces")}
                    <td>{crate.interfaces.join(", ")}</td>
                {/if}
                {#if cols_shown.includes("license")}
                    <td>{crate.license}</td>
                {/if}
                {#if cols_shown.includes("downloads")}
                    <td style="text-align: right">{crate.downloads}</td>
                {/if}
                {#if cols_shown.includes("updated_at")}
                    <td>{new Date(crate.updated_at).toLocaleDateString()}</td>
                {/if}
                {#if cols_shown.includes("created_at")}
                    <td>{new Date(crate.created_at).toLocaleDateString()}</td>
                {/if}
                {#if cols_shown.includes("rust_version")}
                    <td>{crate.rust_version || ""}</td>
                {/if}
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
