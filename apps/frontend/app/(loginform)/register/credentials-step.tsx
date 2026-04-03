'use client';

import { FormControl, FormField, FormItem, FormLabel, FormMessage } from '@components/ui/form';
import { useFormContext } from 'react-hook-form';
import { LinkComponent } from '@components/ui/link';
import Link from 'next/link';
import { LoginIcon } from '@components/ui/login-icon';
import { IconArrowRight, IconUserPlus } from '@tabler/icons-react';
import { Input } from '@components/ui/input';
import { Button } from '@components/ui/button';
import { Alert, AlertDescription, AlertTitle } from '@components/ui/alert';

interface CredentialsStepProps {
    onBack: () => void;
    onSubmit: (e: React.FormEvent) => void;
    isPending: boolean;
    error: string | null;
}

export function CredentialsStep({ onBack, onSubmit, isPending, error }: CredentialsStepProps) {
    const form = useFormContext();

    return (
        <form className="flex flex-col items-center" onSubmit={onSubmit}>
            <LoginIcon>
                <IconUserPlus />
            </LoginIcon>
            <div className="mt-4 flex flex-col gap-1">
                <h1 className="font-semibold text-xl text-center">Set up your credentials</h1>
                <p className="text-sm text-center text-muted-foreground">
                    Choose a username and password
                </p>
            </div>
            <div className="w-sm mt-6 flex flex-col gap-4">
                {error && (
                    <Alert variant="destructive">
                        <AlertTitle>Registration Failed</AlertTitle>
                        <AlertDescription>{error}</AlertDescription>
                    </Alert>
                )}
                <FormField
                    control={form.control}
                    name="preferred_username"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Username</FormLabel>
                            <FormControl>
                                <Input placeholder="johndoe" autoFocus {...field} />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <FormField
                    control={form.control}
                    name="email"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Email</FormLabel>
                            <FormControl>
                                <Input placeholder="john@example.com" type="email" {...field} />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <FormField
                    control={form.control}
                    name="password"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Password</FormLabel>
                            <FormControl>
                                <Input
                                    placeholder="Enter your password"
                                    type="password"
                                    {...field}
                                />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <FormField
                    control={form.control}
                    name="confirm_password"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Confirm Password</FormLabel>
                            <FormControl>
                                <Input
                                    placeholder="Confirm your password"
                                    type="password"
                                    {...field}
                                />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <div className="flex gap-3">
                    <Button
                        type="button"
                        variant="outline"
                        className="flex-1"
                        onClick={onBack}
                    >
                        Back
                    </Button>
                    <Button type="submit" className="flex-1" disabled={isPending}>
                        Create Account <IconArrowRight />
                    </Button>
                </div>
                <div className="text-muted-foreground text-center text-sm">
                    Already have an account?{' '}
                    <LinkComponent>
                        <Link href="/login">Sign In</Link>
                    </LinkComponent>
                </div>
            </div>
        </form>
    );
}
