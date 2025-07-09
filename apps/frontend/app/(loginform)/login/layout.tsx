'use client';
import { Form } from '@components/ui/form';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import z from 'zod';

export const formSchema = z.object({
    username: z.string().min(1, 'Username is required'),
    password: z.string().min(1, 'Password is required'),
});

export type FormSchema = z.infer<typeof formSchema>;

export default function LoginLayout({ children }: { children: React.ReactNode }) {
    const form = useForm<FormSchema>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            username: '',
            password: '',
        },
    });

    return <Form {...form}>{children}</Form>;
}
