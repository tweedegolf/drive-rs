<script lang="ts">

    import {crates, indexes} from './full-crate-db.json'
    import {open_filter} from './store/FilterStore.svelte';
    import CrateList from "./lib/CrateList.svelte";
    import type {FullCrate, Indexes} from "./crate-db";
    import Filter from "./lib/Filter.svelte";
    import SortFilter from './lib/SortFilter.svelte';
    import ForkMe from "./lib/ForkMe.svelte";
    import TextFilter from "./lib/TextFilter.svelte";

    export let BUILD_DATE: string;

    const t_crates = crates as FullCrate[];
    let t_indexes = indexes as Indexes;

    let selected_d: number[][] = [];
    let selected_l: number[][] = [];
    let selected_r: number[][] = [];
    let selected_i: number[][] = [];
    let selected_f: number[][] = [];

    function combine_filters(crate_length: number, selected: any): number[] {
        let selected_crates = Array.from({length: crate_length}, (_, i) => i + 1);

        for (const filter of selected) {
            if (filter === undefined || filter.length === 0) {
                continue;
            }

            let selected_filter = filter.flat();
            selected_crates = selected_crates.filter(crate => selected_filter.includes(crate));
        }

        return selected_crates;
    }

    // Close filter with Esc key
    function handle_key_down(e: KeyboardEvent) {
        if (e.code == "Escape") {
            $open_filter = "";
        }
    }

    $: selected_crates = combine_filters(t_crates.length, [selected_d, selected_l, selected_r, selected_i, selected_f]);

</script>

<svelte:window on:keydown={handle_key_down}/>

<ForkMe/>

<h1>{selected_crates.length} awesome drivers waiting for you!</h1>
<main>
    <div class="filters">
        <SortFilter name="Sort by" />
        <TextFilter crates={t_crates} bind:selected={selected_f}/>
        <Filter name="Dependencies" values={t_indexes.dependencies} bind:selected={selected_d}/>
        <Filter name="Interfaces" values={t_indexes.interfaces} bind:selected={selected_i}/>
        <Filter name="👮 License" values={t_indexes.license} bind:selected={selected_l}/>
        <Filter name="Rust Version" values={t_indexes.rust_version} bind:selected={selected_r}/>
    </div>

    <CrateList crates={t_crates} filter={selected_crates}/>

    <div>Page generated at: {BUILD_DATE}</div>
</main>
