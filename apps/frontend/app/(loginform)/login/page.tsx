'use client';

import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from '@components/ui/form';
import { Input } from '@components/ui/input';
import { LoginIcon } from '@components/ui/login-icon';
import { IconArrowRight, IconKey } from '@tabler/icons-react';
import { useFormContext } from 'react-hook-form';
import { FormSchema } from './layout';
import { Button } from '@components/ui/button';
import { Separator } from '@components/ui/separator';
import { LinkComponent } from '@components/ui/link';
import Link from 'next/link';

export default function Page() {
    const form = useFormContext<FormSchema>();

    return (
        <Form {...form}>
            <LoginIcon>
                <IconKey />
            </LoginIcon>
            <div className="mt-4 flex flex-col gap-1">
                <h1 className="font-semibold text-xl text-center">Sign in to Agin</h1>
                <p className="text-sm text-center text-muted-foreground">
                    Welcome back! Please sign in to continue
                </p>
            </div>
            <div className="w-sm mt-6 flex flex-col gap-4">
                <FormField
                    control={form.control}
                    name="username"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Username or Email</FormLabel>
                            <FormControl>
                                <Input placeholder="Enter your username or email" {...field} />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />
                <Button>
                    Next <IconArrowRight />
                </Button>
                <div className="flex items-center gap-2 pointer-events-none">
                    <Separator className="flex-1" />
                    <div className="text-xs text-muted-foreground font-semibold">OR</div>
                    <Separator className="flex-1" />
                </div>
                <div className="flex flex-col gap-3">
                    <Button variant="outline">
                        <IconKey />
                        Use a security key
                    </Button>
                </div>
                <div className="text-muted-foreground text-center text-sm">
                    Don't have an account?{' '}
                    <LinkComponent>
                        <Link href="/register">Sign Up</Link>
                    </LinkComponent>
                </div>
            </div>
            <div className="text-muted-foreground text-xs absolute left-4 right-4 bottom-4 text-center">
                By signing in you accept our{' '}
                <LinkComponent>
                    <a>Privacy Policy</a>
                </LinkComponent>
            </div>
        </Form>
    );
}
