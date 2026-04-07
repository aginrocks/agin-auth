'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { motion } from 'motion/react';
import { $api } from '@lib/providers/api';
import { IconLayoutGrid, IconSettings, IconLogout, IconChevronDown } from '@tabler/icons-react';
import Link from 'next/link';
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from '@components/ui/dropdown-menu';
import { Button } from '@components/ui/button';

function AppCard({ name, icon }: { name: string; icon?: string | null }) {
    return (
        <div className="flex flex-col items-center gap-3 rounded-xl border border-border bg-card p-6 hover:bg-accent/50 transition-colors cursor-pointer">
            {icon ? (
                <img src={icon} alt={name} className="size-12 rounded-lg" />
            ) : (
                <div className="size-12 rounded-lg bg-primary/10 flex items-center justify-center">
                    <span className="text-lg font-semibold text-primary">
                        {name.charAt(0).toUpperCase()}
                    </span>
                </div>
            )}
            <span className="text-sm font-medium text-foreground">{name}</span>
        </div>
    );
}

function PortalSkeleton() {
    return (
        <div className="min-h-screen bg-background flex items-center justify-center">
            <div className="size-8 rounded-full border-2 border-primary border-t-transparent animate-spin" />
        </div>
    );
}

export default function HomePage() {
    const router = useRouter();

    const profileQuery = $api.useQuery(
        'get',
        '/api/settings/profile',
        {},
        {
            retry: false,
            refetchOnWindowFocus: false,
        }
    );

    const appsQuery = $api.useQuery(
        'get',
        '/api/applications',
        {},
        {
            retry: false,
            refetchOnWindowFocus: false,
            enabled: !!profileQuery.data,
        }
    );

    useEffect(() => {
        const err = profileQuery.error as unknown as { error?: string } | null;
        if (err?.error === 'Unauthorized') {
            router.replace('/login');
        }
    }, [profileQuery.error, router]);

    const logoutMutation = $api.useMutation('post', '/api/logout', {
        onSuccess: () => router.replace('/login'),
    });

    if (profileQuery.isLoading || (!profileQuery.data && !profileQuery.error)) {
        return <PortalSkeleton />;
    }

    const profile = profileQuery.data;
    if (!profile) return null;

    const apps = appsQuery.data ?? [];

    return (
        <div className="min-h-screen bg-background">
            <div className="mx-auto max-w-3xl px-6 py-14">
                <motion.div
                    initial={{ opacity: 0, y: 8 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ duration: 0.3 }}
                    className="space-y-8"
                >
                    {/* Header */}
                    <div className="flex items-center justify-between">
                        <h1 className="text-2xl font-semibold">Applications</h1>
                        <DropdownMenu>
                            <DropdownMenuTrigger asChild>
                                <Button variant="outline" size="sm" className="gap-2">
                                    <div className="size-6 rounded-full bg-primary/10 flex items-center justify-center shrink-0">
                                        <span className="text-xs font-semibold text-primary">
                                            {profile.display_name.charAt(0).toUpperCase()}
                                        </span>
                                    </div>
                                    <span className="max-w-[120px] truncate">{profile.display_name}</span>
                                    <IconChevronDown className="size-4 text-muted-foreground" />
                                </Button>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent align="end" className="w-52">
                                <DropdownMenuLabel className="font-normal">
                                    <p className="text-sm font-medium">{profile.display_name}</p>
                                    <p className="text-xs text-muted-foreground truncate">{profile.email}</p>
                                </DropdownMenuLabel>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem asChild>
                                    <Link href="/dashboard" className="flex items-center gap-2 cursor-pointer">
                                        <IconSettings className="size-4" />
                                        Account Settings
                                    </Link>
                                </DropdownMenuItem>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem
                                    variant="destructive"
                                    disabled={logoutMutation.isPending}
                                    onClick={() => logoutMutation.mutate({})}
                                >
                                    <IconLogout className="size-4" />
                                    Log out
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </div>

                    {/* Applications */}
                    <div>
                        <div className="flex items-center gap-2 mb-4">
                            <IconLayoutGrid className="size-5 text-muted-foreground" />
                            <h2 className="text-lg font-medium">Your applications</h2>
                        </div>
                        {apps.length > 0 ? (
                            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
                                {apps.map((app) => (
                                    <AppCard key={app.slug} name={app.name} icon={app.icon} />
                                ))}
                            </div>
                        ) : (
                            <div className="rounded-xl border border-dashed border-border p-8 text-center">
                                <IconLayoutGrid className="size-8 text-muted-foreground/50 mx-auto mb-2" />
                                <p className="text-sm text-muted-foreground">
                                    No applications available
                                </p>
                            </div>
                        )}
                    </div>
                </motion.div>
            </div>
        </div>
    );
}
