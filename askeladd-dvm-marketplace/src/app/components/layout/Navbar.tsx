"use client";

import Link from 'next/link';
import React, { useState } from 'react';
const Navbar: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  return (
    <nav className=" dark:bg-dark-purple w-full z-20 top-0 start-0 border-b border-gray-200 dark:border-gray-600">
      <div className="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
        <Link href="/" className="flex items-center space-x-3 rtl:space-x-reverse">
          {/* <img src="/vercel.svg" className="h-8" alt="ASKELADD" width={50} height={50} /> */}
          <span className="self-center text-md font-semibold whitespace-nowrap dark:text-white">ASKELADD</span>
        </Link>
        <div className="flex md:order-2 space-x-3 md:space-x-0 rtl:space-x-reverse">
          <button
            aria-expanded={isOpen}
            onClick={() => setIsOpen(!isOpen)}  // Toggle the state on click
            data-collapse-toggle="navbar-sticky" type="button" className="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600" aria-controls="navbar-sticky">
            <span className="sr-only">Open main menu</span>
            <svg className="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 17 14">
              <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M1 1h15M1 7h15M1 13h15" />
            </svg>
          </button>
        </div>
        {/* <div className={`items-center justify-between w-full md:flex md:w-auto md:order-1 ${isOpen ? 'flex' : 'hidden'}`} id="navbar-sticky"> */}

        <div className={`gap-5 items-center justify-between ${isOpen ? 'flex' : 'hidden'} w-full md:flex md:w-auto md:order-1`} id="navbar-sticky">
          <ul className="flex flex-col p-4 md:p-0 mt-4 font-medium rounded-lg md:space-x-8 rtl:space-x-reverse md:flex-row md:mt-0 md:border-0 dark:border-gray-700">
            <li className='my-5'>
              <Link href={"/stwo-program"}
                onClick={() => setIsOpen(!isOpen)}
                className="nav-button-link rounded m-5 text-white">STWO Program</Link>
            </li>
            <li className='my-5'>
              <Link href={"/config-marketplace"}
                onClick={() => setIsOpen(!isOpen)}
                className="nav-button-link rounded m-5 text-white">DVM ZK Config</Link>
            </li>

            <li className='my-5'>
              <Link href={"/launch-program"}
                onClick={() => setIsOpen(!isOpen)}
                className="nav-button-link rounded m-5 text-white">Launch program</Link>
            </li>
          </ul>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;
