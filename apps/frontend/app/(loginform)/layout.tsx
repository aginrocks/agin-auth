import Image from 'next/image';

export default function LoginLayout({ children }: { children: React.ReactNode }) {
    return (
        <div className="flex h-screen p-2 gap-2 justify-center">
            <div className="flex-1 w-full max-w-xl flex flex-col justify-center items-center h-full relative mx-4">
                {children}
            </div>
            <div className="flex-1 relative hidden md:block">
                <Image src="/background.jpg" alt="" fill className="object-cover rounded-xl" />
            </div>
        </div>
    );
}
