export function DashboardSkeleton() {
    return (
        <div className="rounded-2xl border border-border bg-card">
            {[...Array(5)].map((_, i) => (
                <div key={i} className={`px-5 py-4 flex items-center gap-4 ${i < 4 ? 'border-b border-border/60' : ''}`}>
                    <div className="size-5 rounded bg-muted animate-pulse shrink-0" />
                    <div className="flex-1 space-y-2">
                        <div className="h-3.5 w-28 rounded bg-muted animate-pulse" />
                        <div className="h-3 w-40 rounded bg-muted animate-pulse" />
                    </div>
                </div>
            ))}
        </div>
    );
}

export function DashboardError() {
    return (
        <div className="rounded-2xl border border-border bg-card px-5 py-4">
            <p className="text-sm text-destructive">Failed to load - make sure you are signed in.</p>
        </div>
    );
}

export function DashboardWarning({ message }: { message: string }) {
    return (
        <div className="rounded-2xl border border-border bg-card px-5 py-4">
            <p className="text-sm text-muted-foreground">{message}</p>
        </div>
    );
}
