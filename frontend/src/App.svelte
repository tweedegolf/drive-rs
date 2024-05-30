<script lang="ts">
    import {crates, indexes} from './website_db.json'
    import CrateList from "./lib/CrateList.svelte";
    import type {Crate, Indexes} from "./crate-db";
    import Filter from "./lib/Filter.svelte";

    const t_crates = crates as Crate[];
    let t_indexes = indexes as Indexes;

    let selected_d: number[][] = [];
    let selected_l: number[][] = [];
    let selected_r: number[][] = [];

    function combine_filters(crate_length: number, selected: any): number[] {
        let selected_crates = Array.from({length: crate_length}, (_, i) => i + 1);

        console.log(selected);

        for (const filter of selected) {
            if (filter === undefined || filter.length === 0) {
                continue;
            }

            let selected_filter = filter.flat();
            selected_crates = selected_crates.filter(crate => selected_filter.includes(crate));
        }

        console.log(selected_crates);

        return selected_crates;
    }

    $: selected_crates = combine_filters(t_crates.length, [selected_d, selected_l, selected_r]);
</script>

<main class="grid-container">
    <div>
        <Filter name="Dependencies" values={t_indexes.dependencies} bind:selected={selected_d}/>
        <Filter name="👮 License" values={t_indexes.license} bind:selected={selected_l}/>
        <Filter name="Rust Version" values={t_indexes.rust_version} bind:selected={selected_r}/>
    </div>

    <CrateList crates={t_crates} filter={selected_crates}/>
</main>

<style>
    .grid-container {
        display: grid;
        grid-template-columns: 1fr 3fr;
        gap: 20px;
    }
</style>
