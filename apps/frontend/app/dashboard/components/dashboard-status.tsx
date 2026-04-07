function SkeletonCard({ rows }: { rows: number }) {
    return (
        <div className="rounded-2xl border border-border bg-card">
            {[...Array(rows)].map((_, i) => (
                <div key={i} className={`px-5 py-4 flex items-center gap-4 ${i < rows - 1 ? 'border-b border-border/60' : ''}`}>
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

export function DashboardSkeleton() {
    return (
        <>
            <div className="mb-6">
                <div className="h-4 w-12 rounded bg-muted animate-pulse mb-3" />
                <SkeletonCard rows={1} />
            </div>
            <div className="mb-6">
                <div className="h-4 w-16 rounded bg-muted animate-pulse mb-3" />
                <SkeletonCard rows={5} />
            </div>
            <div className="mt-8">
                <div className="h-4 w-24 rounded bg-muted/60 animate-pulse mb-3" />
                <div className="rounded-2xl border border-destructive/20 bg-card">
                    <div className="px-5 py-4 flex items-center gap-4">
                        <div className="size-5 rounded bg-muted animate-pulse shrink-0" />
                        <div className="flex-1 space-y-2">
                            <div className="h-3.5 w-28 rounded bg-muted animate-pulse" />
                            <div className="h-3 w-40 rounded bg-muted animate-pulse" />
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}

export function SessionsSkeleton() {
    return (
        <div className="rounded-2xl border border-border bg-card">
            {[...Array(3)].map((_, i) => (
                <div
                    key={i}
                    className={`px-5 py-4 flex items-center gap-4 ${i < 2 ? 'border-b border-border/60' : ''}`}
                >
                    <div className="size-5 rounded bg-muted animate-pulse shrink-0" />
                    <div className="flex-1 space-y-2">
                        <div className="h-3.5 w-32 rounded bg-muted animate-pulse" />
                        <div className="h-3 w-48 rounded bg-muted animate-pulse" />
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
