/* eslint-disable @typescript-eslint/ban-ts-comment */
/* eslint-disable @typescript-eslint/prefer-ts-expect-error */

import { useEffect, useRef, useState } from 'react'
import { useInterval } from 'usehooks-ts'
import _ from 'lodash'

interface Props {
    content: string
    speed?: number
    variability?: number
}

function getNewText(printed: string, content: string) {
    // eslint-disable-next-line no-useless-escape
    let escapedPrinted = printed.replace(/\$/g, '\\$')
    escapedPrinted = escapedPrinted.replace(/'/g, '\\\'')
    escapedPrinted = escapedPrinted.replace(/\+/g, '\\+')
    escapedPrinted = escapedPrinted.replace(/\[/g, '\\[')
    escapedPrinted = escapedPrinted.replace(/\]/g, '\\]')
    escapedPrinted = escapedPrinted.replace(/\(/g, '\\(')
    escapedPrinted = escapedPrinted.replace(/\)/g, '\\)')
    const pattern = `^${escapedPrinted}([\\s\\S]*)`
    const regex = new RegExp(pattern)
    const results = regex.exec(content)
    if (results == null) { return '' }
    return results[1]
}

export default function Terminal(props: Props) {
    const bottomRef = useRef(null)
    const [isShown, setIsShown] = useState(true)

    // * what is printed, as a string, not split into arrays
    const [printedContentString, setPrintedContentString] = useState('')
    // * the shown content, split into arrays line by line
    const [presentationContent, setPresentationContent] = useState<string[]>([])

    // * how fast the terminal should print
    const [delay, setDelay] = useState(50)
    // * is the terminal currently printing, meaning is there a work queue
    const [isPlaying, setIsPlaying] = useState(true)

    useInterval(
        () => {
            // * figure out what text is new since we last were here
            const newText = getNewText(printedContentString, props.content)

            // * get out of dodge if we didn't get anything; non-op
            if (newText == null || newText === '') {
                // @ts-ignore
                bottomRef.current?.scrollIntoView({ behavior: 'auto' })
                setIsPlaying(false)
                return
            }

            if (newText[0] == null) return

            // * we type one letter at a time
            const nextLetter = newText[0]

            // * we're going to be fussing with state, make a copy
            const localPresentationContent = _.cloneDeep(presentationContent)

            // * if we pulled a linefeed off the stack, we need to make a new element in the array
            // * which is map'd over to generate the content of the terminal
            if (nextLetter.includes('\n')) {
                // * here's that new array element
                localPresentationContent.push('')
                // * we're about to be done, so set the side effects this routine needs to touch
                setPrintedContentString(localPresentationContent.join('\n'))
                setPresentationContent(localPresentationContent)
                return
            }

            // * add the new letter to the end of the last line
            if (localPresentationContent.length > 0) {
                // * we're going to append to the end of the string of the last element
                localPresentationContent[localPresentationContent.length - 1] += nextLetter
                // * or we're going to make a new element if there isn't one
            } else localPresentationContent.push(nextLetter)

            // * we take in the controls for typing speed
            const variability = props.variability ?? 1.5
            const generalSpeed = props.speed ?? 4
            // * we use an exponential function to map the random parameter to the output speed
            setDelay(Math.exp(Math.random() * variability) * generalSpeed)
            // * set the expected side effects
            setPrintedContentString(localPresentationContent.join('\n'))
            setPresentationContent(localPresentationContent)

            // @ts-ignore
            bottomRef.current?.scrollIntoView({ behavior: 'auto' })
        },
        isPlaying ? delay : null
    )

    useEffect(() => {
        setIsPlaying(true)
        setIsShown(true)
    }, [props.content])

    // * red/close button handler
    function close() {
        setIsShown(false)
    }

    const backgroundColor = 'bg-[#000000bb]'

    return (
        <>
            {isShown && (
                <div className="z-[40] absolute top-[17%] left-[5%] w-[85%] mx-auto drop-shadow-[10px_10px_15px_rgba(0,0,0,0.5)] ">
                    <div className={'w-full shadow-2xl subpixel-antialiased rounded h-[75vh] ' + backgroundColor + ' border-black mx-auto'}>
                        <div className="flex items-center h-6 rounded-t bg-gray-100 border-b border-gray-500 text-center text-black" id="headerTerminal">
                            <div className="flex ml-2 items-center text-center border-red-900 bg-red-500 shadow-inner rounded-full w-3 h-3" id="closebtn" onClick={close}>
                            </div>
                            <div className="ml-2 border-yellow-900 bg-yellow-500 shadow-inner rounded-full w-3 h-3" id="minbtn">
                            </div>
                            <div className="ml-2 border-green-900 bg-green-500 shadow-inner rounded-full w-3 h-3" id="maxbtn">
                            </div>
                            <div className="mx-auto pr-16" id="terminaltitle">
                                <p className="text-center text-sm">Terminal</p>
                            </div>

                        </div>
                        <div className={'overflow-auto pl-1 pt-1 h-[72vh] transition-all text-green-200 duration-[5000ms] font-mono text-xs  '} id="console">
                            {presentationContent.map((line, index) => (
                                <p key={index} className="pb-1">
                                    <span className='fading'>
                                        {line}
                                    </span>
                                    {index === presentationContent.length - 1 && (
                                        <span className='blink text-green-200'> █</span>
                                    )}
                                </p>
                            ))}
                            <div ref={bottomRef} />
                        </div>
                    </div>
                </div>
            )}
        </>
    )
}