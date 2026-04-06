'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { IconUser, IconDevices } from '@tabler/icons-react';

const navItems = [
    { href: '/dashboard', label: 'User details', icon: IconUser },
    { href: '/dashboard/sessions', label: 'Sessions', icon: IconDevices },
];

export default function DashboardLayout({ children }: { children: React.ReactNode }) {
    const pathname = usePathname();

    return (
        <div className="min-h-screen bg-background">
            <div className="mx-auto max-w-4xl px-6 py-14">
                <div className="flex flex-col gap-8 md:flex-row">
                    <nav className="shrink-0 md:w-48">
                        <ul className="flex gap-2 overflow-x-auto pb-1 md:block md:space-y-1">
                            {navItems.map((item) => {
                                const active = item.href === '/dashboard'
                                    ? pathname === '/dashboard'
                                    : pathname.startsWith(item.href);
                                return (
                                    <li key={item.href}>
                                        <Link
                                            href={item.href}
                                            className={`flex min-w-max items-center gap-2 rounded-md px-3 py-2 text-sm transition-colors ${
                                                active
                                                    ? 'bg-accent text-accent-foreground font-medium'
                                                    : 'text-muted-foreground hover:text-foreground hover:bg-accent/50'
                                            }`}
                                        >
                                            <item.icon className="size-4" />
                                            {item.label}
                                        </Link>
                                    </li>
                                );
                            })}
                        </ul>
                    </nav>
                    <main className="flex-1 min-w-0">{children}</main>
                </div>
            </div>
        </div>
    );
}
