/**
 * A restore-able command represented with its full output
 */
export interface FullHistoryRecord {
    sequence_counter: number,
    command: string,
    output?: string | null,
    exit_code?: number | null,
    ran_by: string,
    created_at: string,
}

/**
 * A compact command representation used to quickly and efficiently display a list of commands
 */
export interface HistoryRecord {
    sequence_counter: number,
    command: string,
    exit_code?: number | null,
    ran_by: string,
    created_at: string,
}