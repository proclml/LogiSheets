import { SelectedCell } from './events'
import { Subscription, Subject } from 'rxjs'
import { debounceTime } from 'rxjs/operators'
import styles from './canvas.module.scss'
import {
    MouseEvent,
    ReactElement,
    useEffect,
    useRef,
    useState,
    FC,
} from 'react'
import {
    useSelector,
    useStartCell,
    useScrollbar,
    useDnd,
    useText,
    Render,
    useHighlightCell,
    useResizers,
} from './managers'
import { Cell, match } from './defs'
import {
    ScrollbarComponent
} from '@/components/scrollbar'
import { EventType, on } from '@/common/events'
import { ContextmenuComponent } from './contextmenu'
import { SelectorComponent } from '@/components/selector'
import { ResizerComponent } from '@/components/resize'
import { BlurEvent, TextContainerComponent } from '@/components/textarea'
import { DndComponent } from '@/components/dnd'
import { InvalidFormulaComponent } from './invalid-formula'
import { Buttons } from '@/common'
import { CellInputBuilder } from '@/api'
import { DialogComponent } from '@/ui/dialog'
import { useInjection } from '@/core/ioc/provider'
import { Backend, DataService, SheetService } from '@/core/data'
import { TYPES } from '@/core/ioc/types'
export const OFFSET = 100

export interface CanvasProps {
    selectedCell$: (e: SelectedCell) => void
}

export const CanvasComponent: FC<CanvasProps> = ({ selectedCell$ }) => {
    const [contextmenuOpen, setContextMenuOpen] = useState(false)
    const [contextMenuEl, setContextMenu] = useState<ReactElement>()
    const BACKEND_SERVICE = useInjection<Backend>(TYPES.Backend)
    const SHEET_SERVICE = useInjection<SheetService>(TYPES.Sheet)
    const DATA_SERVICE = useInjection<DataService>(TYPES.Data)
    const renderMng = useInjection<Render>(TYPES.Render)

    const canvasEl = useRef<HTMLCanvasElement>(null)

    const startCellMng = useStartCell()
    const selectorMng = useSelector()
    const scrollbarMng = useScrollbar()
    const dndMng = useDnd()
    const textMng = useText()
    const highlights = useHighlightCell()
    const resizerMng = useResizers()
    const focus$ = useRef(new Subject<void>())

    useEffect(() => {
        const subs = new Subscription()
        const _canvasEl = canvasEl.current as HTMLCanvasElement
        subs.add(BACKEND_SERVICE.render$.subscribe(() => {
            renderMng.render(_canvasEl)
        }))
        subs.add(on(window, EventType.RESIZE)
            .pipe(debounceTime(100))
            .subscribe(() => {
                renderMng.render(_canvasEl)
            }))
        subs.add(renderMng.rendered$.subscribe(() => {
            // resizer manager??????viewRange
            resizerMng.init()
        }))
        // ???????????????
        subs.add(startCellMng.startCellEvent$.current.subscribe(e => {
            selectorMng.startCellChange(e)
            textMng.startCellChange(e)
            if (e === undefined || e.same)
                return
            if (e.cell.type !== 'Cell')
                return
            const { startRow: row, startCol: col } = e.cell.coodinate
            selectedCell$({ row, col })
        }))
        return () => {
            subs.unsubscribe()
        }
    }, [renderMng.rendered$])
    // ???????????????????????????state?????????useeffect???????????????
    useEffect(() => {
        const subs = new Subscription()
        subs.add(on(window, EventType.RESIZE)
            .pipe(debounceTime(100))
            .subscribe(() => {
                scrollbarMng.resize(canvasEl.current as HTMLCanvasElement)
            }))
        return () => {
            subs.unsubscribe()
        }
    })

    // ?????????
    useEffect(() => {
        selectorMng.init(canvasEl.current as HTMLCanvasElement)
        scrollbarMng.initScrollbar()
        textMng.init(canvasEl.current as HTMLCanvasElement)
        startCellMng.canvasChange()
        DATA_SERVICE.sendDisplayArea()
    }, [canvasEl])

    useEffect(() => {
        if (selectorMng.startCell)
            dndMng.selectorChange({
                canvas: canvasEl.current as HTMLCanvasElement,
                start: selectorMng.startCell,
                end: selectorMng.endCell,
            })
    }, [selectorMng.selector])

    // ????????????????????????
    useEffect(() => {
        if (!textMng.editing)
            return
        highlights.init(textMng.currText.current)
    }, [textMng.editing])

    // ????????????
    useEffect(() => {
        renderMng.render(canvasEl.current as HTMLCanvasElement)
        startCellMng.scroll()
    }, [scrollbarMng.xScrollbarAttr, scrollbarMng.yScrollbarAttr])

    const onMousedown = async (e: MouseEvent) => {
        e.stopPropagation()
        e.preventDefault()
        const mousedown = async () => {
            const isBlur = await textMng.blur()
            if (!isBlur) {
                focus$.current.next()
                return
            }
            if (e.buttons === Buttons.RIGHT)
                return
            const matchCell = match(e.clientX, e.clientY, canvasEl.current as HTMLCanvasElement, DATA_SERVICE.cachedViewRange)
            const isResize = resizerMng.mousedown(e.nativeEvent, canvasEl.current as HTMLCanvasElement)
            if (isResize)
                return
            const isDnd = dndMng.onMouseDown(e.nativeEvent)
            if (isDnd)
                return
            startCellMng.mousedown(e, matchCell, canvasEl.current as HTMLCanvasElement)
        }
        mousedown()
        const sub = new Subscription()
        sub.add(on(window, EventType.MOUSE_UP).subscribe(() => {
            dndMng.onMouseUp()
            resizerMng.mouseup()
            sub.unsubscribe()
        }))
        sub.add(on(window, EventType.MOUSE_MOVE).subscribe(mme => {
            mme.preventDefault()
            const startCell = match(mme.clientX, mme.clientY, canvasEl.current as HTMLCanvasElement, DATA_SERVICE.cachedViewRange)
            const isResize = resizerMng.mousemove(mme)
            if (isResize)
                return
            if (startCellMng.startCell.current?.equals(startCell) === false) {
                const isDnd = dndMng.onMouseMove(mme, canvasEl.current as HTMLCanvasElement, startCell, selectorMng.endCell ?? startCell)
                if (isDnd)
                    return
            }
            selectorMng.onMouseMove(startCell)
        }))
    }

    const blur = (e: BlurEvent<Cell>) => {
        const oldText = textMng.context?.text ?? ''
        textMng.blur()
        highlights.blur()
        if (e.bindingData === undefined)
            return
        const newText = textMng.currText.current.trim()
        if (oldText === newText)
            return
        const payload = new CellInputBuilder()
            .row(e.bindingData.coodinate.startRow)
            .col(e.bindingData.coodinate.startCol)
            .sheetIdx(SHEET_SERVICE.getActiveSheet())
            .input(newText)
            .build()
        BACKEND_SERVICE.sendTransaction([payload])
    }

    const onContextMenu = (e: MouseEvent) => {
        e.preventDefault()
        e.stopPropagation()
        const matchCell = match(e.clientX, e.clientY, canvasEl.current as HTMLCanvasElement, DATA_SERVICE.cachedViewRange)
        startCellMng.mousedown(e, matchCell, canvasEl.current as HTMLCanvasElement, selectorMng.selector)
        if (!selectorMng.startCell)
            return
        setContextMenu((
            <ContextmenuComponent
                isOpen={true}
                mouseevent={e}
                setIsOpen={setContextMenuOpen}
                startCell={selectorMng.startCell}
                endCell={selectorMng.endCell}
            ></ContextmenuComponent>
        ))
        setContextMenuOpen(true)
    }
    const type = (text: string) => {
        textMng.currText.current = text
        highlights.update(text)
    }

    return (<div
        onContextMenu={e => onContextMenu(e)}
        onMouseDown={onMousedown}
        className={styles.host}
    >
        <canvas
            className={styles.canvas}
            ref={canvasEl}
            onWheel={scrollbarMng.mouseWheel}
        >????????????????????????canvas?????????????????????</canvas>
        {contextmenuOpen && contextMenuEl ? contextMenuEl : null}
        {selectorMng.selector ? (
            <SelectorComponent
                {...selectorMng.selector}
            ></SelectorComponent>
        ) : null}
        <ScrollbarComponent
            {...scrollbarMng.xScrollbarAttr}
            setScrollDistance={e => scrollbarMng.setScrollDistance(e, 'x')}
        ></ScrollbarComponent>
        <ScrollbarComponent
            {...scrollbarMng.yScrollbarAttr}
            setScrollDistance={e => scrollbarMng.setScrollDistance(e, 'y')}
        ></ScrollbarComponent>
        {textMng.context && textMng.editing ?
            <TextContainerComponent
                context={textMng.context}
                blur={blur}
                type={type}
                checkFormula={textMng.checkFormula}
                focus$={focus$.current}
            ></TextContainerComponent>
            : null}
        {dndMng.range ? <DndComponent
            dragging={dndMng.dragging !== undefined}
            x={dndMng.range.startCol}
            y={dndMng.range.startRow}
            width={dndMng.range.width}
            height={dndMng.range.height}
            draggingX={dndMng.dragging?.startCol}
            draggingY={dndMng.dragging?.startRow}
        ></DndComponent> : null}
        <DialogComponent
            content={<InvalidFormulaComponent
                close$={() => textMng.setValidFormulaOpen(false)}
            ></InvalidFormulaComponent>}
            close$={() => textMng.setValidFormulaOpen(false)}
            isOpen={textMng.validFormulaOpen}
        ></DialogComponent>
        {
            highlights.highlightCells.map((cell, i) => {
                const cellStyle = cell.style
                return <div
                    className={styles['highlight-cell']}
                    style={{
                        left: cellStyle.x,
                        top: cellStyle.y,
                        width: `${cellStyle.width}px`,
                        height: `${cellStyle.height}px`,
                        backgroundColor: cellStyle.bgColor.css(),
                    }}
                    key={i}
                ></div>
            })
        }
        {
            resizerMng.resizers.map((resizer, i) => {
                const { startCol: x, startRow: y, width, height } = resizer.range
                const { isRow } = resizer
                const rect = (canvasEl.current as HTMLCanvasElement).getBoundingClientRect()
                return <ResizerComponent
                    hoverText={resizerMng.hoverText}
                    x={!isRow ? x : 0}
                    y={!isRow ? 0 : y}
                    width={width}
                    height={height}
                    key={i}
                    movingX={!isRow ? resizerMng.moving.x : 0}
                    movingY={isRow ? resizerMng.moving.y : 0}
                    movingHeight={!isRow ? rect.height : 0}
                    movingWidth={!isRow ? 0 : rect.width}
                    active={resizerMng.active === resizer}
                    type={isRow ? 'row' : 'col'}
                ></ResizerComponent>
            })
        }
    </div>
    )
}

export * from './events'
export * from './defs'