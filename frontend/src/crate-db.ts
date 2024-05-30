
export interface Root {
    crates: Crate[]
    indexes: Indexes
}

export interface Crate {
    name: string
    description: string
    version: string
    license: string
    downloads: number
    updated_at: string
    created_at: string
    rust_version?: string
    dependencies: Dependency[]
}

export interface Dependency {
    name: string
    version: string
}

export interface Indexes {
    license: { [key: string]: number[] }
    rust_version: { [key: string]: number[] }
    dependencies: { [key: string]: number[] }
}
