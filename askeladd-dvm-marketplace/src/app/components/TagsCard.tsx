import React from 'react';
// Define the props for the component
interface TagsCardProps {
    tags: string[][];  // Array of array of strings
}
const TagsCard: React.FC<TagsCardProps> = ({ tags }) => {
    return (
        <div className="p-4 bg-white shadow-lg rounded-lg ">
            {tags.map((tagGroup, index) => (
                <div key={index} className="flex flex-wrap gap-2 mb-2 overflow-x">
                     {index}  
                    {tagGroup.map((tag, idx) => (
                        // <span key={idx} className="bg-gray-200 text-sm px-2 py-1 rounded text-sm sm:text-base md:text-lg text-gray-800 overflow-hidden overflow-ellipsis whitespace-nowrap">
                     <span key={idx} className="bg-gray-200 text-sm px-2 py-1 rounded text-sm sm:text-base md:text-lg text-gray-800 overflow-hidden whitespace-nowrap">
                            {tag}
                        </span>
                    ))}
                </div>
            ))}
        </div>
    );
};

export default TagsCard;
