'use client';

import { FormControl, FormField, FormItem, FormLabel, FormMessage } from '@components/ui/form';
import { useFormContext } from 'react-hook-form';
import { LinkComponent } from '@components/ui/link';
import Link from 'next/link';
import { LoginIcon } from '@components/ui/login-icon';
import { IconArrowRight, IconUserPlus } from '@tabler/icons-react';
import { Input } from '@components/ui/input';
import { Button } from '@components/ui/button';

interface InfoStepProps {
    onNext: () => void;
}

export function InfoStep({ onNext }: InfoStepProps) {
    const form = useFormContext();

    return (
        <div className="flex flex-col items-center">
            <LoginIcon>
                <IconUserPlus />
            </LoginIcon>
            <div className="mt-4 flex flex-col gap-1">
                <h1 className="font-semibold text-xl text-center">Create your account</h1>
                <p className="text-sm text-center text-muted-foreground">
                    Tell us a bit about yourself
                </p>
            </div>
            <div className="w-sm mt-6 flex flex-col gap-4">
                <div className="flex gap-3">
                    <FormField
                        control={form.control}
                        name="first_name"
                        render={({ field }) => (
                            <FormItem className="flex-1">
                                <FormLabel>First Name</FormLabel>
                                <FormControl>
                                    <Input placeholder="John" autoFocus {...field} />
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                    <FormField
                        control={form.control}
                        name="last_name"
                        render={({ field }) => (
                            <FormItem className="flex-1">
                                <FormLabel>Last Name</FormLabel>
                                <FormControl>
                                    <Input placeholder="Doe" {...field} />
                                </FormControl>
                                <FormMessage />
                            </FormItem>
                        )}
                    />
                </div>
                <FormField
                    control={form.control}
                    name="display_name"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Display Name</FormLabel>
                            <FormControl>
                                <Input placeholder="John Doe" {...field} />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <Button type="button" onClick={onNext}>
                    Next <IconArrowRight />
                </Button>
                <div className="text-muted-foreground text-center text-sm">
                    Already have an account?{' '}
                    <LinkComponent>
                        <Link href="/login">Sign In</Link>
                    </LinkComponent>
                </div>
            </div>
        </div>
    );
}
