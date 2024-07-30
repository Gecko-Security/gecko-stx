export declare function vfs(action: string, data: unknown): Promise<boolean> | Promise<string> | Promise<{
    [k: string]: string;
}> | Promise<void>;
