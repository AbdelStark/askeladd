import { NDKEvent } from '@nostr-dev-kit/ndk';
import { Event as NostrEvent } from 'nostr-tools';
import React, { useState } from 'react';
import TagsCard from './TagsCard';
// Define the props for the component
interface TagsCardProps {
    event: NDKEvent | NostrEvent;  // Array of array of strings
}
const EventCard: React.FC<TagsCardProps> = ({ event }) => {

    const [seeTag, setSeeTag] = useState<boolean>(false)


    const date:string|undefined = event?.created_at ? new Date(event?.created_at).toDateString() : undefined
    return (
        // <div className="max-w-md mx-auto max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
        <div className="mx-auto max-w-lg p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
            <div className="">
                {date &&
                    <p className="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white leading-normal">{date}</p>
                }
                <p className="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">Job id: {event?.kind}</p>
                <p className="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{event?.content}</p>
                <button onClick={() => {
                    setSeeTag(!seeTag)
                }}>See tags</button>
                {seeTag &&
                    <TagsCard tags={event?.tags}></TagsCard>
                }
            </div>


        </div>
    );
};

export default EventCard;
